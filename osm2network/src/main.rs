use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;

use osmpbf::{Element, ElementReader};
use serde::{Deserialize, Serialize};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Give a .osm.pbf as input");
    }

    println!("Scraping {}", args[1]);
    let (nodes, ways) = scrape_elements(&args[1]);
    println!("Got {} nodes and {} ways", nodes.len(), ways.len());

    let network = split_edges(nodes, ways);
    println!("Got {} edges", network.edges.len());

    {
        println!("Saving to network.bin");
        let writer = BufWriter::new(File::create("network.bin").unwrap());
        bincode::serialize_into(writer, &network).unwrap();
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Position {
    // in decimicrodegrees (10⁻⁷)
    lon: i32,
    lat: i32,
}

struct Way {
    tags: Vec<(String, String)>,
    nodes: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
struct Network {
    // Keyed by a pair of node IDs
    edges: HashMap<(i64, i64), Edge>,
}

#[derive(Serialize, Deserialize)]
struct Edge {
    way_id: i64,
    tags: Vec<(String, String)>,
    geometry: Vec<Position>,
}

fn scrape_elements(path: &str) -> (HashMap<i64, Position>, HashMap<i64, Way>) {
    // Scrape every node ID -> position
    let mut nodes = HashMap::new();
    // Scrape every routable road. Just tags and node lists to start.
    let mut ways = HashMap::new();

    let reader = ElementReader::from_path(path).unwrap();
    // TODO par_map_reduce would be fine if we can merge the hashmaps; there should be no repeated
    // keys
    reader
        .for_each(|element| {
            match element {
                Element::Node(node) => {
                    nodes.insert(
                        node.id(),
                        Position {
                            lon: node.decimicro_lon(),
                            lat: node.decimicro_lat(),
                        },
                    );
                }
                Element::DenseNode(node) => {
                    nodes.insert(
                        node.id(),
                        Position {
                            lon: node.decimicro_lon(),
                            lat: node.decimicro_lat(),
                        },
                    );
                }
                Element::Way(way) => {
                    // TODO Improve filtering
                    if way.tags().any(|(key, _)| key == "highway") {
                        ways.insert(
                            way.id(),
                            Way {
                                tags: way
                                    .tags()
                                    .map(|(k, v)| (k.to_string(), v.to_string()))
                                    .collect(),
                                nodes: way.refs().collect(),
                            },
                        );
                    }
                }
                Element::Relation(_) => {}
            }
        })
        .unwrap();

    (nodes, ways)
}

fn split_edges(nodes: HashMap<i64, Position>, ways: HashMap<i64, Way>) -> Network {
    // Count how many ways reference each node
    let mut node_counter: HashMap<i64, usize> = HashMap::new();
    for way in ways.values() {
        for node in &way.nodes {
            *node_counter.entry(*node).or_insert(0) += 1;
        }
    }

    // Split each way into edges
    let mut edges = HashMap::new();
    for (way_id, way) in ways {
        let mut node1 = way.nodes[0];
        let mut pts = Vec::new();

        let num_nodes = way.nodes.len();
        for (idx, node) in way.nodes.into_iter().enumerate() {
            pts.push(nodes[&node]);
            // Edges start/end at intersections between two ways. The endpoints of the way also
            // count as intersections.
            let is_endpoint =
                idx == 0 || idx == num_nodes - 1 || *node_counter.get(&node).unwrap() > 1;
            if is_endpoint && pts.len() > 1 {
                edges.insert(
                    (node1, node),
                    Edge {
                        way_id,
                        tags: way.tags.clone(),
                        geometry: std::mem::take(&mut pts),
                    },
                );

                // Start the next edge
                node1 = node;
                pts.push(nodes[&node]);
            }
        }
    }

    Network { edges }
}
