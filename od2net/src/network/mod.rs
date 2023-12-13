mod amenities;
mod create_from_osm;
mod greenspace;
mod output;

use std::collections::HashMap;
use std::io::BufReader;

use anyhow::Result;
use fs_err::File;
use osm_reader::{NodeID, WayID};
use serde::{Deserialize, Serialize};

use lts::{Tags, LTS};

#[derive(Serialize, Deserialize)]
pub struct Network {
    // TODO Doesn't handle multiple edges between the same node pair
    pub edges: HashMap<(NodeID, NodeID), Edge>,
    pub intersections: HashMap<NodeID, Position>,
}

// TODO Rename this. We don't represent counts, but instead summed uptake. If every single route we
// considered would actually happen, then this would be equivalent to counts.
pub struct Counts {
    // TODO Don't use f64 -- we'll end up rounding somewhere anyway, so pick a precision upfront.
    pub count_per_edge: HashMap<(NodeID, NodeID), f64>,
    pub errors: u64,

    // Count how many times a point is used successfully as an origin or destination
    pub count_per_origin: HashMap<Position, f64>,
    pub count_per_destination: HashMap<Position, f64>,

    // In meters. Indexed by LTS as u8
    pub total_distance_by_lts: [f64; 5],
}

impl Counts {
    pub fn new() -> Self {
        Self {
            count_per_edge: HashMap::new(),
            errors: 0,

            count_per_origin: HashMap::new(),
            count_per_destination: HashMap::new(),

            total_distance_by_lts: [0.0; 5],
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
        for i in 0..5 {
            self.total_distance_by_lts[i] += other.total_distance_by_lts[i];
        }
    }
}

impl Network {
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

#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub way_id: WayID,
    pub tags: Tags,
    geometry: Vec<Position>,
    // Storing the derived field is negligible for file size
    pub length_meters: f64,
    // LTS is often incorporated in cost, but is also used for visualization. It's useful to
    // conceptually separate these.
    pub lts: LTS,
    // TODO Option is weird -- we should upfront filter this out.
    pub cost: Option<usize>,
    // TODO Maybe generalize as a cost and a bunch of properties per edge -- like proximity
    // modifiers for greenspace, lighting, commercial areas
    pub nearby_amenities: usize,
}
