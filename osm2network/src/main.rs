use std::collections::HashMap;

use osmpbf::{Element, ElementReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Give a .osm.pbf as input");
    }

    let (nodes, ways) = scrape_elements(&args[1]);
    println!("Got {} nodes and {} ways", nodes.len(), ways.len());

    // split_up_roads. wind up with (node1, node2) to a (duplicated??) segment structure
}

struct Node {
    // in decimicrodegrees (10⁻⁷)
    lon: i32,
    lat: i32,
}

struct Way {
    tags: Vec<(String, String)>,
    nodes: Vec<i64>,
}

fn scrape_elements(path: &str) -> (HashMap<i64, Node>, HashMap<i64, Way>) {
    // Scrape every node ID -> position
    let mut nodes: HashMap<i64, Node> = HashMap::new();
    // Scrape every routable road. Just tags and node lists to start.
    let mut ways: HashMap<i64, Way> = HashMap::new();

    let reader = ElementReader::from_path(path).unwrap();
    // TODO par_map_reduce would be fine if we can merge the hashmaps; there should be no repeated
    // keys
    reader
        .for_each(|element| {
            match element {
                Element::Node(node) => {
                    nodes.insert(
                        node.id(),
                        Node {
                            lon: node.decimicro_lon(),
                            lat: node.decimicro_lat(),
                        },
                    );
                }
                Element::DenseNode(node) => {
                    nodes.insert(
                        node.id(),
                        Node {
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
