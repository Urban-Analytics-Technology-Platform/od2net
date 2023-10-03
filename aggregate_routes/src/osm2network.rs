use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Write};

use anyhow::Result;
use fs_err::File;
use geo::prelude::HaversineLength;
use geo::LineString;
use geojson::{feature::Id, Feature, FeatureWriter, Geometry, JsonObject, JsonValue, Value};
use indicatif::{HumanCount, ProgressBar, ProgressStyle};
use osmpbf::{Element, ElementReader};
use serde::{Deserialize, Serialize};

use super::config::LtsMapping;
use super::plugins;
use super::timer::Timer;
use super::OutputMetadata;
use lts::{Tags, LTS};

#[derive(Serialize, Deserialize)]
pub struct Network {
    // Keyed by a pair of node IDs
    // TODO Doesn't handle multiple edges between the same node pair
    pub edges: HashMap<(i64, i64), Edge>,
    // Node IDs that're above
    pub intersections: HashMap<i64, Position>,
}

// TODO Rename this. We don't represent counts, but instead summed uptake. If every single route we
// considered would actually happen, then this would be equivalent to counts.
pub struct Counts {
    // TODO Don't use f64 -- we'll end up rounding somewhere anyway, so pick a precision upfront.
    pub count_per_edge: HashMap<(i64, i64), f64>,
    pub errors: u64,

    // Count how many times a point is used successfully as an origin or destination
    pub count_per_origin: HashMap<Position, f64>,
    pub count_per_destination: HashMap<Position, f64>,
}

impl Counts {
    pub fn new() -> Self {
        Self {
            count_per_edge: HashMap::new(),
            errors: 0,

            count_per_origin: HashMap::new(),
            count_per_destination: HashMap::new(),
        }
    }

    /// Adds other to this one
    pub fn combine(&mut self, other: Counts) {
        self.errors += other.errors;
        for (key, count) in other.count_per_edge {
            *self.count_per_edge.entry(key).or_insert(0.0) += count;
        }
        for (key, count) in other.count_per_origin {
            *self.count_per_origin.entry(key).or_insert(0.0) += count;
        }
        for (key, count) in other.count_per_destination {
            *self.count_per_destination.entry(key).or_insert(0.0) += count;
        }
    }
}

impl Network {
    pub fn make_from_pbf(
        osm_pbf_path: &str,
        bin_path: &str,
        lts: &LtsMapping,
        timer: &mut Timer,
    ) -> Result<Network> {
        timer.start("Make Network from pbf");
        timer.start("Scrape OSM data");
        let (nodes, ways) = scrape_elements(osm_pbf_path)?;
        timer.stop();
        println!(
            "  Got {} nodes and {} ways",
            HumanCount(nodes.len() as u64),
            HumanCount(ways.len() as u64)
        );

        timer.start("Split into edges");
        let mut network = split_edges(nodes, ways);
        timer.stop();
        println!(
            "  Split into {} edges",
            HumanCount(network.edges.len() as u64),
        );

        timer.start("Calculate LTS for all edges");
        // TODO Refactor helper?
        let progress = ProgressBar::new(network.edges.len() as u64).with_style(ProgressStyle::with_template(
                "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap());
        // LTS calculations can have high overhead in one case, so calculate them in batches
        let all_keys: Vec<(i64, i64)> = network.edges.keys().cloned().collect();
        for key_batch in all_keys.chunks(1000) {
            let tags_batch: Vec<Tags> = key_batch
                .iter()
                .map(|e| network.edges[&e].cleaned_tags())
                .collect();
            let lts_batch = calculate_lts_batch(lts, tags_batch);
            for (key, lts) in key_batch.into_iter().zip(lts_batch) {
                progress.inc(1);
                network.edges.get_mut(&key).unwrap().lts = lts;
            }
        }
        timer.stop();

        timer.start(format!("Saving to {bin_path}"));
        let writer = BufWriter::new(File::create(bin_path)?);
        bincode::serialize_into(writer, &network)?;
        timer.stop();

        timer.stop();
        Ok(network)
    }

    pub fn load_from_bin(path: &str) -> Result<Network> {
        let network = bincode::deserialize_from(BufReader::new(File::open(path)?))?;
        Ok(network)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    // in decimicrodegrees (10⁻⁷)
    lon: i32,
    lat: i32,
}

impl Position {
    pub fn from_degrees(lon: f64, lat: f64) -> Self {
        // TODO Rounding? Unit test bidirectionality
        Self {
            lon: (lon * 1e7) as i32,
            lat: (lat * 1e7) as i32,
        }
    }

    // TODO Degrees?
    pub fn to_degrees(self) -> (f64, f64) {
        (1e-7 * self.lon as f64, 1e-7 * self.lat as f64)
    }

    fn to_degrees_vec(self) -> Vec<f64> {
        // Round here, since this one is used for GJ output
        vec![
            trim_f64(1e-7 * self.lon as f64),
            trim_f64(1e-7 * self.lat as f64),
        ]
    }

    pub fn to_degrees_array(self) -> [f64; 2] {
        [1e-7 * self.lon as f64, 1e-7 * self.lat as f64]
    }
}

fn trim_f64(x: f64) -> f64 {
    (x * 10e6).round() / 10e6
}

struct Way {
    tags: Vec<(String, String)>,
    nodes: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub way_id: i64,
    // TODO Why not store Tags? Could even serialize as this
    tags: Vec<(String, String)>,
    geometry: Vec<Position>,
    // Storing the derived field is negligible for file size
    pub length_meters: f64,
    lts: LTS,
}

impl Edge {
    fn to_geojson(
        &self,
        node1: i64,
        node2: i64,
        count: f64,
        id: usize,
        output_osm_tags: bool,
    ) -> Feature {
        let geometry = Geometry::new(Value::LineString(
            self.geometry.iter().map(|pt| pt.to_degrees_vec()).collect(),
        ));
        let mut properties = JsonObject::new();
        if output_osm_tags {
            let mut tags = JsonObject::new();
            for (key, value) in &self.tags {
                tags.insert(key.to_string(), JsonValue::from(value.to_string()));
            }
            properties.insert("osm_tags".to_string(), tags.into());
        }
        properties.insert("node1".to_string(), JsonValue::from(node1));
        properties.insert("node2".to_string(), JsonValue::from(node2));
        properties.insert("way".to_string(), JsonValue::from(self.way_id));
        properties.insert("count".to_string(), JsonValue::from(count));
        properties.insert("lts".to_string(), serde_json::to_value(self.lts).unwrap());
        Feature {
            bbox: None,
            geometry: Some(geometry),
            id: Some(Id::Number(id.into())),
            properties: Some(properties),
            foreign_members: None,
        }
    }

    pub fn to_geojson_for_detailed_output(
        &self,
        node1: i64,
        node2: i64,
        geometry_forwards: bool,
    ) -> Feature {
        let mut pts = self
            .geometry
            .iter()
            .map(|pt| pt.to_degrees_vec())
            .collect::<Vec<_>>();
        if !geometry_forwards {
            pts.reverse();
        }
        let geometry = Geometry::new(Value::LineString(pts));

        let mut properties = JsonObject::new();
        let mut tags = JsonObject::new();
        for (key, value) in &self.tags {
            tags.insert(key.to_string(), JsonValue::from(value.to_string()));
        }
        properties.insert("osm_tags".to_string(), tags.into());
        properties.insert("node1".to_string(), JsonValue::from(node1));
        properties.insert("node2".to_string(), JsonValue::from(node2));
        properties.insert("way".to_string(), JsonValue::from(self.way_id));
        properties.insert("lts".to_string(), serde_json::to_value(self.lts).unwrap());
        Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
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
                        // Temporary
                        lts: LTS::NotAllowed,
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
    pub fn write_geojson(
        &self,
        path: &str,
        counts: Counts,
        output_od_points: bool,
        output_osm_tags: bool,
        output_metadata: &OutputMetadata,
    ) -> Result<()> {
        // Write one feature at a time to avoid memory problems
        let mut writer = FeatureWriter::from_writer(BufWriter::new(File::create(path)?));
        writer.write_foreign_member("config", output_metadata)?;

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
                let feature = edge.to_geojson(node1, node2, count, id_counter, output_osm_tags);
                writer.write_feature(&feature)?;
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

        if output_od_points {
            // Also write origin/destination points with the number of routes to the same file. It
            // hugely bloats the size, but keeping them together is useful right now.

            for (key, counter) in [
                ("origin_count", counts.count_per_origin),
                ("destination_count", counts.count_per_destination),
            ] {
                for (pt, count) in counter {
                    id_counter += 1;
                    let geometry = Geometry::new(Value::Point(pt.to_degrees_vec()));
                    let mut properties = JsonObject::new();
                    properties.insert(key.to_string(), JsonValue::from(count));
                    writer.write_feature(&Feature {
                        bbox: None,
                        geometry: Some(geometry),
                        id: Some(Id::Number(id_counter.into())),
                        properties: Some(properties),
                        foreign_members: None,
                    })?;
                }
            }
        }

        writer.finish()?;

        Ok(())
    }

    pub fn write_csv(&self, path: &str, counts: &Counts) -> Result<()> {
        let mut file = BufWriter::new(File::create(path)?);
        writeln!(file, "way,node1,node2,count")?;

        let mut skipped = 0;
        for ((node1, node2), count) in &counts.count_per_edge {
            if let Some(edge) = self
                .edges
                .get(&(*node1, *node2))
                .or_else(|| self.edges.get(&(*node2, *node1)))
            {
                let way = edge.way_id;
                writeln!(file, "{way},{node1},{node2},{count}")?;
            } else {
                skipped += 1;
            }
        }

        println!(
            "Skipped {} edges (started/ended mid-edge)",
            HumanCount(skipped)
        );
        Ok(())
    }
}

fn calculate_length_meters(pts: &[Position]) -> f64 {
    let line_string =
        LineString::<f64>::from(pts.iter().map(|pt| pt.to_degrees()).collect::<Vec<_>>());
    line_string.haversine_length()
}

fn calculate_lts_batch(lts: &LtsMapping, tags_batch: Vec<Tags>) -> Vec<LTS> {
    match lts {
        LtsMapping::SpeedLimitOnly => tags_batch
            .into_iter()
            .map(|tags| lts::speed_limit_only(tags).0)
            .collect(),
        LtsMapping::BikeOttawa => tags_batch
            .into_iter()
            .map(|tags| lts::bike_ottawa(tags).0)
            .collect(),
        LtsMapping::ExternalCommand(command) => {
            plugins::custom_lts::external_command(command, tags_batch).unwrap()
        }
    }
}
