use std::io::{BufReader, BufWriter};

use anyhow::Result;
use fast_paths::{FastGraph, InputGraph, PathCalculator};
use fs_err::File;
use indicatif::ParallelProgressIterator;
use osm_reader::NodeID;
use rayon::prelude::*;
use rstar::primitives::GeomWithData;
use rstar::RTree;
use serde::{Deserialize, Serialize};

use super::config::Uptake;
use super::network::{Counts, Network, Position};
use super::node_map::{deserialize_nodemap, NodeMap};
use super::plugins::uptake;
use super::requests::Request;
use super::timer::Timer;
use super::utils;

// TODO Vary ch_path with CostFunction
pub fn run(
    ch_path: &str,
    network: &Network,
    requests: Vec<Request>,
    uptake: &Uptake,
    timer: &mut Timer,
) -> Result<Counts> {
    let prepared_ch = build_ch(ch_path, network, timer)?;
    let closest_intersection = build_closest_intersection(network, &prepared_ch.node_map, timer);

    let progress = utils::progress_bar_for_count(requests.len());
    let num_requests = requests.len();

    let counts = requests
        .into_par_iter()
        // Split the work evenly among CPUs. Otherwise rayon fold too eagerly splits, creating too
        // many PerThreadStates in-memory. See
        // https://users.rust-lang.org/t/rayon-with-expensive-to-construct-combine-accumulator/78252/3.
        .with_min_len(num_requests / num_cpus::get())
        .progress_with(progress)
        .fold(PerThreadState::new, |mut acc, req| {
            if acc.path_calc.is_none() {
                acc.path_calc = Some(fast_paths::create_calculator(&prepared_ch.ch));
            }
            handle_request(
                req,
                &mut acc.counts,
                acc.path_calc.as_mut().unwrap(),
                &closest_intersection,
                &prepared_ch,
                uptake,
                network,
            );
            acc
        })
        .reduce_with(|mut acc1, acc2| {
            acc1.counts.combine(acc2.counts);
            acc1
        })
        .unwrap()
        .counts;

    Ok(counts)
}

struct PerThreadState {
    counts: Counts,
    path_calc: Option<PathCalculator>,
}

impl PerThreadState {
    fn new() -> Self {
        Self {
            counts: Counts::new(),
            path_calc: None,
        }
    }
}

pub fn handle_request(
    req: Request,
    counts: &mut Counts,
    path_calc: &mut fast_paths::PathCalculator,
    closest_intersection: &RTree<IntersectionLocation>,
    prepared_ch: &PreparedCH,
    uptake: &Uptake,
    network: &Network,
) {
    let start = closest_intersection
        .nearest_neighbor(&[req.x1, req.y1])
        .unwrap()
        .data;
    let end = closest_intersection
        .nearest_neighbor(&[req.x2, req.y2])
        .unwrap()
        .data;
    if start == end {
        counts.errors_same_endpoints.push(req);
        return;
    }

    // A sanity check that snapping works -- manually check these:
    if false {
        debug!(
            "req from {}, {} -> {}, {} snaps to http://openstreetmap.org/node/{}",
            req.x1,
            req.y1,
            req.x2,
            req.y2,
            prepared_ch.node_map.translate_id(start)
        );
    }

    let Some(path) = path_calc.calc_path(&prepared_ch.ch, start, end) else {
        counts.errors_no_path.push(req);
        return;
    };

    let mut per_od_counts = if let (Some(origin), Some(destination)) = (req.origin, req.destination)
    {
        Some(
            counts
                .count_per_od
                .entry((origin, destination))
                .or_insert_with(|| (0.0, [0.0; 5])),
        )
    } else {
        None
    };

    // fast_paths returns the total cost, but it's not necessarily the right unit. Calculate how
    // long this route is.
    let mut total_distance = 0.0;
    for pair in path.get_nodes().windows(2) {
        let i1 = prepared_ch.node_map.translate_id(pair[0]);
        let i2 = prepared_ch.node_map.translate_id(pair[1]);
        let edge = network
            .edges
            .get(&(i1, i2))
            .or_else(|| network.edges.get(&(i2, i1)))
            .unwrap();
        total_distance += edge.length_meters;

        // TODO Is it weird to update this if the uptake is 0?
        counts.total_distance_by_lts[edge.lts as u8 as usize] += edge.length_meters;
        if let Some(ref mut per_od) = per_od_counts {
            per_od.1[edge.lts as u8 as usize] += edge.length_meters;
        }
    }

    let count = uptake::calculate_uptake(uptake, total_distance);
    // TODO Pick an epsilon based on the final rounding we do... though it's possible 1e6 trips
    // cross a segment each with probability 1e-6?
    if count == 0.0 {
        return;
    }

    for pair in path.get_nodes().windows(2) {
        // TODO Actually, don't do this translation until the very end
        let i1 = prepared_ch.node_map.translate_id(pair[0]);
        let i2 = prepared_ch.node_map.translate_id(pair[1]);
        *counts.count_per_edge.entry((i1, i2)).or_insert(0.0) += count;
    }

    *counts
        .count_per_origin
        .entry(Position::from_degrees(req.x1, req.y1))
        .or_insert(0.0) += count;
    *counts
        .count_per_destination
        .entry(Position::from_degrees(req.x2, req.y2))
        .or_insert(0.0) += count;

    if let Some(per_od) = per_od_counts {
        per_od.0 += count;
    }
}

#[derive(Serialize, Deserialize)]
pub struct PreparedCH {
    pub ch: FastGraph,
    #[serde(deserialize_with = "deserialize_nodemap")]
    pub node_map: NodeMap<NodeID>,
}

pub fn build_ch(path: &str, network: &Network, timer: &mut Timer) -> Result<PreparedCH> {
    println!("Trying to load CH from {path}");
    match File::open(path)
        .map_err(|err| err.into())
        .and_then(|f| bincode::deserialize_from(BufReader::new(f)))
    {
        Ok(ch) => {
            return Ok(ch);
        }
        Err(err) => {
            println!("That failed, so regenerating it: {err}");
        }
    }

    let result = just_build_ch(network, timer);
    let writer = BufWriter::new(File::create(path)?);
    bincode::serialize_into(writer, &result)?;
    Ok(result)
}

// No IO
pub fn just_build_ch(network: &Network, timer: &mut Timer) -> PreparedCH {
    timer.start("Building InputGraph");
    let mut input_graph = InputGraph::new();
    let mut node_map = NodeMap::new();
    for ((node1, node2), edge) in &network.edges {
        // Put every node in the CH, even if we wind up with no edges there
        let node1 = node_map.get_or_insert(*node1);
        let node2 = node_map.get_or_insert(*node2);

        if let (Some(forward_cost), Some(backward_cost)) = (edge.forward_cost, edge.backward_cost) {
            // Everything bidirectional for now!
            input_graph.add_edge(node1, node2, forward_cost);
            input_graph.add_edge(node2, node1, backward_cost);
        }
    }
    input_graph.freeze();
    timer.stop();

    timer.start("Preparing the CH");
    let ch = fast_paths::prepare(&input_graph);
    timer.stop();

    PreparedCH { ch, node_map }
}

// fast_paths ID representing the OSM node ID as the data
// TODO We may be able to override the distance function? Does it work with WGS84?
pub type IntersectionLocation = GeomWithData<[f64; 2], usize>;

pub fn build_closest_intersection(
    network: &Network,
    node_map: &NodeMap<NodeID>,
    timer: &mut Timer,
) -> RTree<IntersectionLocation> {
    timer.start("Building RTree for matching request points to OSM nodes");
    let mut points = Vec::new();
    for (id, pt) in &network.intersections {
        points.push(IntersectionLocation::new(
            pt.to_degrees_array(),
            node_map.get(*id),
        ));
    }
    let rtree = RTree::bulk_load(points);
    timer.stop();
    rtree
}
