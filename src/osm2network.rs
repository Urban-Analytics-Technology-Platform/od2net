use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Write};
use std::time::Instant;

use anyhow::Result;
use fs_err::File;
use geo::prelude::HaversineLength;
use geo::LineString;
use geojson::{feature::Id, Feature, Geometry, JsonObject, JsonValue, Value};
use indicatif::HumanCount;
use osmpbf::{Element, ElementReader};
use serde::{Deserialize, Serialize};

use super::tags::Tags;

#[derive(Serialize, Deserialize)]
pub struct Network {
    // Keyed by a pair of node IDs
    pub edges: HashMap<(i64, i64), Edge>,
    // Node IDs that're above
    pub intersections: HashMap<i64, Position>,
}

pub struct Counts {
    pub count_per_edge: HashMap<(i64, i64), usize>,
    pub errors: u64,
    pub filtered_out: u64,
}

impl Counts {
    pub fn new() -> Self {
        Self {
            count_per_edge: HashMap::new(),
            errors: 0,
            filtered_out: 0,
        }
    }
}

impl Network {
    pub fn make_from_pbf(osm_pbf_path: &str, bin_path: &str) -> Result<Network> {
        let mut start = Instant::now();
        let (nodes, ways) = scrape_elements(osm_pbf_path)?;
        println!(
            "  Got {} nodes and {} ways. That took {:?}",
            HumanCount(nodes.len() as u64),
            HumanCount(ways.len() as u64),
            Instant::now().duration_since(start)
        );

        start = Instant::now();
        let network = split_edges(nodes, ways);
        println!(
            "  Split into {} edges. That took {:?}",
            HumanCount(network.edges.len() as u64),
            start
        );

        println!("  Saving to {bin_path}");
        start = Instant::now();
        let writer = BufWriter::new(File::create(bin_path)?);
        bincode::serialize_into(writer, &network)?;
        println!("  That took {:?}", Instant::now().duration_since(start));

        Ok(network)
    }

    pub fn load_from_bin(path: &str) -> Result<Network> {
        let network = bincode::deserialize_from(BufReader::new(File::open(path)?))?;
        Ok(network)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    // in decimicrodegrees (10⁻⁷)
    lon: i32,
    lat: i32,
}

impl Position {
    // TODO Degrees?
    pub fn to_degrees(self) -> (f64, f64) {
        (1e-7 * self.lon as f64, 1e-7 * self.lat as f64)
    }

    pub fn to_degrees_vec(self) -> Vec<f64> {
        vec![1e-7 * self.lon as f64, 1e-7 * self.lat as f64]
    }

    pub fn to_degrees_array(self) -> [f64; 2] {
        [1e-7 * self.lon as f64, 1e-7 * self.lat as f64]
    }
}

struct Way {
    tags: Vec<(String, String)>,
    nodes: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Edge {
    way_id: i64,
    tags: Vec<(String, String)>,
    geometry: Vec<Position>,
    // Storing the derived field is negligible for file size
    pub length_meters: f64,
}

impl Edge {
    fn to_geojson(&self, node1: i64, node2: i64, count: usize, id: usize) -> Feature {
        let geometry = Geometry::new(Value::LineString(
            self.geometry.iter().map(|pt| pt.to_degrees_vec()).collect(),
        ));
        let mut properties = JsonObject::new();
        for (key, value) in &self.tags {
            properties.insert(key.to_string(), JsonValue::from(value.to_string()));
        }
        properties.insert("node1".to_string(), JsonValue::from(node1));
        properties.insert("node2".to_string(), JsonValue::from(node2));
        properties.insert("way".to_string(), JsonValue::from(self.way_id));
        properties.insert("count".to_string(), JsonValue::from(count));
        properties.insert(
            "lts".to_string(),
            JsonValue::from(self.level_traffic_stress()),
        );
        Feature {
            bbox: None,
            geometry: Some(geometry),
            id: Some(Id::Number(id.into())),
            properties: Some(properties),
            foreign_members: None,
        }
    }

    pub fn cleaned_tags(&self) -> Tags {
        let mut tags = Tags::new();
        for (k, v) in &self.tags {
            tags.insert(k, v);
        }
        tags
    }

    // 1 suitable for kids, 4 high stress, 0 is unknown. Need to swap this out for something much
    // better, and maybe make it directional!
    fn level_traffic_stress(&self) -> usize {
        // TODO Handle bicycle=no, on things like highway=footway

        let tags = self.cleaned_tags();

        if let Some(mph) = tags
            .get("maxspeed")
            .and_then(|x| x.trim_end_matches(" mph").parse::<usize>().ok())
        {
            if mph <= 20 {
                return 2;
            }
            if mph >= 40 {
                return 4;
            }
            // Between 20 and 40
            return 3;
        }

        /*if tags.is("highway", "residential") {
            return 1;
        }*/

        0 // TODO unknown
    }
}

fn scrape_elements(path: &str) -> Result<(HashMap<i64, Position>, HashMap<i64, Way>)> {
    // Scrape every node ID -> position
    let mut nodes = HashMap::new();
    // Scrape every routable road. Just tags and node lists to start.
    let mut ways = HashMap::new();

    let reader = ElementReader::from_path(path)?;
    // TODO par_map_reduce would be fine if we can merge the hashmaps; there should be no repeated
    // keys
    reader.for_each(|element| {
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
    })?;

    Ok((nodes, ways))
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
    let mut intersections = HashMap::new();
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
                intersections.insert(node1, pts[0]);
                intersections.insert(node, *pts.last().unwrap());
                let length_meters = calculate_length_meters(&pts);
                edges.insert(
                    (node1, node),
                    Edge {
                        way_id,
                        tags: way.tags.clone(),
                        geometry: std::mem::take(&mut pts),
                        length_meters,
                    },
                );

                // Start the next edge
                node1 = node;
                pts.push(nodes[&node]);
            }
        }
    }

    Network {
        edges,
        intersections,
    }
}

impl Network {
    pub fn write_geojson(&self, path: &str, counts: Counts) -> Result<()> {
        // Write one feature at a time manually, to avoid memory problems
        let mut file = BufWriter::new(File::create(path)?);
        writeln!(file, "{{\"type\":\"FeatureCollection\", \"features\":[")?;
        let mut add_comma = false;

        let mut skipped = 0;
        let mut id_counter = 0;
        for ((node1, node2), count) in counts.count_per_edge {
            // TODO Track forwards and backwards counts separately, and optionally merge later?
            if let Some(edge) = self
                .edges
                .get(&(node1, node2))
                .or_else(|| self.edges.get(&(node2, node1)))
            {
                id_counter += 1;
                if add_comma {
                    writeln!(file, ",")?;
                } else {
                    add_comma = true;
                }
                let feature = edge.to_geojson(node1, node2, count, id_counter);
                // TODO Trim f64 precision for some savings
                serde_json::to_writer(&mut file, &feature)?;
            } else {
                // TODO We don't handle routes starting or ending in the middle of an edge yet
                //println!("No edge from https://www.openstreetmap.org/node/{node1} to https://www.openstreetmap.org/node/{node2} or vice versa");
                skipped += 1;
            }
        }
        println!(
            "Skipped {} edges (started/ended mid-edge)",
            HumanCount(skipped)
        );

        writeln!(file, "]}}")?;
        Ok(())
    }
}

fn calculate_length_meters(pts: &[Position]) -> f64 {
    let line_string =
        LineString::<f64>::from(pts.iter().map(|pt| pt.to_degrees()).collect::<Vec<_>>());
    line_string.haversine_length()
}
