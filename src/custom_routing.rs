use std::io::{BufReader, BufWriter};
use std::time::Instant;

use anyhow::Result;
use fast_paths::{FastGraph, InputGraph};
use fs_err::File;
use indicatif::{ProgressBar, ProgressStyle};
use rstar::primitives::GeomWithData;
use rstar::RTree;
use serde::{Deserialize, Serialize};

use super::input::CostFunction;
use super::node_map::{deserialize_nodemap, NodeMap};
use super::osm2network::{Counts, Edge, Network};
use super::requests::Request;

// TODO Vary ch_path with CostFunction
pub fn run(
    ch_path: &str,
    network: &Network,
    requests: Vec<Request>,
    cost: CostFunction,
) -> Result<Counts> {
    let prepared_ch = build_ch(ch_path, network, cost)?;
    let closest_intersection = build_closest_intersection(network, &prepared_ch.node_map);

    // Count routes per node pairs
    let progress = ProgressBar::new(requests.len() as u64).with_style(ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap());
    let mut counts = Counts::new();

    let mut path_calc = fast_paths::create_calculator(&prepared_ch.ch);
    for req in requests {
        progress.inc(1);

        let start = closest_intersection
            .nearest_neighbor(&[req.x1, req.y1])
            .unwrap()
            .data;
        let end = closest_intersection
            .nearest_neighbor(&[req.x2, req.y2])
            .unwrap()
            .data;

        // A sanity check that snapping works -- manually check these:
        if false {
            println!(
                "req from {}, {} snaps to http://openstreetmap.org/node/{}",
                req.x1,
                req.y1,
                prepared_ch.node_map.translate_id(start)
            );
        }

        if let Some(path) = path_calc.calc_path(&prepared_ch.ch, start, end) {
            for pair in path.get_nodes().windows(2) {
                // TODO Actually, don't do this translation until the very end
                let i1 = prepared_ch.node_map.translate_id(pair[0]);
                let i2 = prepared_ch.node_map.translate_id(pair[1]);
                *counts.count_per_edge.entry((i1, i2)).or_insert(0) += 1;
            }
        } else {
            counts.errors += 1;
        }
    }
    progress.finish();

    Ok(counts)
}

#[derive(Serialize, Deserialize)]
struct PreparedCH {
    ch: FastGraph,
    #[serde(deserialize_with = "deserialize_nodemap")]
    node_map: NodeMap<i64>,
}

fn build_ch(path: &str, network: &Network, cost: CostFunction) -> Result<PreparedCH> {
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

    let mut start = Instant::now();
    println!("Building InputGraph");
    let mut input_graph = InputGraph::new();
    let mut node_map = NodeMap::new();
    for ((node1, node2), edge) in &network.edges {
        // Put every node in the CH, even if we wind up with no edges there
        let node1 = node_map.get_or_insert(*node1);
        let node2 = node_map.get_or_insert(*node2);

        if let Some(cost) = edge_cost(edge, cost) {
            // Everything bidirectional for now!
            input_graph.add_edge(node1, node2, cost);
            input_graph.add_edge(node2, node1, cost);
        }
    }
    input_graph.freeze();
    println!(
        "That took {:?}. Now preparing the CH",
        Instant::now().duration_since(start)
    );

    start = Instant::now();
    let ch = fast_paths::prepare(&input_graph);
    println!(
        "Preparing the CH took {:?}\n",
        Instant::now().duration_since(start)
    );

    let result = PreparedCH { ch, node_map };
    let writer = BufWriter::new(File::create(path)?);
    bincode::serialize_into(writer, &result)?;
    Ok(result)
}

fn edge_cost(edge: &Edge, cost: CostFunction) -> Option<usize> {
    let tags = edge.cleaned_tags();

    // TODO Match the lts.ts definition
    if tags.is("bicycle", "no") || tags.is("highway", "motorway") || tags.is("highway", "proposed")
    {
        return None;
    }

    let dist = edge.length_meters();

    let output = match cost {
        CostFunction::Distance => dist,
        CostFunction::AvoidMainRoads => {
            // TODO Match the LTS definitoins
            let penalty = if tags.is("highway", "residential") || tags.is("highway", "cycleway") {
                1.0
            } else {
                5.0
            };
            penalty * dist
        }
    };
    Some(output.round() as usize)
}

// fast_paths ID representing the OSM node ID as the data
// TODO We may be able to override the distance function? Does it work with WGS84?
type IntersectionLocation = GeomWithData<[f64; 2], usize>;

fn build_closest_intersection(
    network: &Network,
    node_map: &NodeMap<i64>,
) -> RTree<IntersectionLocation> {
    println!("Building RTree for matching request points to OSM nodes");
    let start = Instant::now();
    let mut points = Vec::new();
    for (id, pt) in &network.intersections {
        points.push(IntersectionLocation::new(
            pt.to_degrees_array(),
            node_map.get(*id),
        ));
    }
    let rtree = RTree::bulk_load(points);
    println!("That took {:?}\n", Instant::now().duration_since(start));
    rtree
}
