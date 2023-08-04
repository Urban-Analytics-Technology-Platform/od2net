use std::time::Instant;

use anyhow::Result;
use fast_paths::{FastGraph, InputGraph};

use super::node_map::NodeMap;
use super::osm2network::{Edge, Network};
use super::requests::Request;

pub fn run(network: Network, requests: Vec<Request>) -> Result<()> {
    // TODO Save and load from a file
    let ch = build_ch(network);

    Ok(())
}

fn build_ch(network: Network) -> FastGraph {
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
    println!("That took {:?}", Instant::now().duration_since(start));

    start = Instant::now();
    let ch = fast_paths::prepare(&input_graph);
    println!(
        "Preparing the CH took {:?}",
        Instant::now().duration_since(start)
    );
    ch
}

fn cost(edge: &Edge) -> usize {
    edge.length_meters().round() as usize
}
