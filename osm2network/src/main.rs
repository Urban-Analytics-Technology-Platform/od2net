use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::time::Instant;

use clap::Parser;
use geojson::{Feature, Geometry, JsonObject, JsonValue, Value};
use osmpbf::{Element, ElementReader};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    // TODO subcommands, really
    #[clap(long)]
    osm_pbf_input: Option<String>,

    #[clap(long)]
    bin_input: Option<String>,

    #[clap(long)]
    counts: Option<String>,
}

fn main() {
    let args = Args::parse();

    let network: Network = if let Some(path) = args.osm_pbf_input {
        println!("Scraping {path}");
        let mut start = Instant::now();
        let (nodes, ways) = scrape_elements(&path);
        println!(
            "Got {} nodes and {} ways. That took {:?}",
            nodes.len(),
            ways.len(),
            Instant::now().duration_since(start)
        );

        start = Instant::now();
        let network = split_edges(nodes, ways);
        println!(
            "Split into {} edges. That took {:?}",
            network.edges.len(),
            start
        );

        println!("Saving to network.bin");
        start = Instant::now();
        let writer = BufWriter::new(File::create("network.bin").unwrap());
        bincode::serialize_into(writer, &network).unwrap();
        println!("That took {:?}", Instant::now().duration_since(start));

        network
    } else if let Some(path) = args.bin_input {
        bincode::deserialize_from(BufReader::new(File::open(path).unwrap())).unwrap()
    } else {
        panic!("Give input");
    };

    println!("Saving to network.geojson");
    let start = Instant::now();
    if let Some(path) = args.counts {
        let counts = bincode::deserialize_from(BufReader::new(File::open(path).unwrap())).unwrap();
        network.write_geojson_with_counts("network.geojson", counts);
    } else {
        network.write_all_geojson("network.geojson");
    }
    println!("That took {:?}", Instant::now().duration_since(start));
}

#[derive(Deserialize)]
struct Counts {
    count_per_edge: HashMap<(i64, i64), usize>,
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

impl Edge {
    fn to_geojson(&self, node1: i64, node2: i64) -> Feature {
        let geometry = Geometry::new(Value::LineString(
            self.geometry
                .iter()
                .map(|pt| vec![1e-7 * pt.lon as f64, 1e-7 * pt.lat as f64])
                .collect(),
        ));
        let mut properties = JsonObject::new();
        for (key, value) in &self.tags {
            properties.insert(key.to_string(), JsonValue::from(value.to_string()));
        }
        properties.insert("node1".to_string(), JsonValue::from(node1));
        properties.insert("node2".to_string(), JsonValue::from(node2));
        properties.insert("way".to_string(), JsonValue::from(self.way_id));
        Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: Some(properties),
            foreign_members: None,
        }
    }
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

impl Network {
    fn write_all_geojson(&self, path: &str) {
        // Write one feature at a time manually, to avoid memory problems
        let mut file = BufWriter::new(File::create(path).unwrap());
        writeln!(file, "{{\"type\":\"FeatureCollection\", \"features\":[").unwrap();
        let mut add_comma = false;
        for ((node1, node2), edge) in &self.edges {
            if add_comma {
                writeln!(file, ",").unwrap();
            } else {
                add_comma = true;
            }
            let feature = edge.to_geojson(*node1, *node2);
            serde_json::to_writer(&mut file, &feature).unwrap();
        }
        writeln!(file, "]}}").unwrap();
    }

    fn write_geojson_with_counts(&self, path: &str, counts: Counts) {
        // Write one feature at a time manually, to avoid memory problems
        let mut file = BufWriter::new(File::create(path).unwrap());
        writeln!(file, "{{\"type\":\"FeatureCollection\", \"features\":[").unwrap();
        let mut add_comma = false;

        for ((node1, node2), count) in counts.count_per_edge {
            if let Some(edge) = self.edges.get(&(node1, node2)) {
                if add_comma {
                    writeln!(file, ",").unwrap();
                } else {
                    add_comma = true;
                }
                let mut feature = edge.to_geojson(node1, node2);
                feature.set_property("count", count);
                serde_json::to_writer(&mut file, &feature).unwrap();
            } else {
                println!("No edge from https://www.openstreetmap.org/node/{node1} to https://www.openstreetmap.org/node/{node2}");
            }
        }
        writeln!(file, "]}}").unwrap();
    }
}
