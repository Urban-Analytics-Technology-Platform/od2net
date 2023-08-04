use std::time::Instant;

use anyhow::Result;
use fast_paths::{FastGraph, InputGraph};
use indicatif::{ProgressBar, ProgressStyle};
use rstar::primitives::GeomWithData;
use rstar::RTree;

use super::node_map::NodeMap;
use super::osm2network::{Counts, Edge, Network};
use super::requests::Request;

pub fn run(network: &Network, requests: Vec<Request>) -> Result<Counts> {
    // TODO Save and load from a file
    let (ch, node_map) = build_ch(network);
    let closest_intersection = build_closest_intersection(network, &node_map);

    // Count routes per node pairs
    let progress = ProgressBar::new(requests.len() as u64).with_style(ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap());
    let mut counts = Counts::new();

    let mut path_calc = fast_paths::create_calculator(&ch);
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

        if let Some(path) = path_calc.calc_path(&ch, start, end) {
            for pair in path.get_nodes().windows(2) {
                let i1 = node_map.translate_id(pair[0]);
                let i2 = node_map.translate_id(pair[1]);
                *counts.count_per_edge.entry((i1, i2)).or_insert(0) += 1;
            }
        } else {
            counts.errors += 1;
        }
    }
    progress.finish();

    Ok(counts)
}

fn build_ch(network: &Network) -> (FastGraph, NodeMap<i64>) {
    let mut start = Instant::now();
    println!("Building InputGraph");
    let mut input_graph = InputGraph::new();
    let mut node_map = NodeMap::new();
    for ((node1, node2), edge) in &network.edges {
        // Everything bidirectional for now!
        input_graph.add_edge(
            node_map.get_or_insert(*node1),
            node_map.get_or_insert(*node2),
            cost(edge),
        );
        input_graph.add_edge(
            node_map.get_or_insert(*node2),
            node_map.get_or_insert(*node1),
            cost(edge),
        );
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
    (ch, node_map)
}

fn cost(edge: &Edge) -> usize {
    edge.length_meters().round() as usize
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
