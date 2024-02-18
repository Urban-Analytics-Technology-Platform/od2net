mod amenities;
mod create_from_osm;
mod greenspace;
mod output;

use std::collections::HashMap;
use std::io::BufReader;
use std::io::{Read, Seek};

use elevation::GeoTiffElevation;
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
    // slope as a percentage.
    pub slope: Option<f64>,
    pub slope_factor: Option<(f64, f64)>,
    // Storing the derived field is negligible for file size
    pub length_meters: f64,
    // LTS is often incorporated in cost, but is also used for visualization. It's useful to
    // conceptually separate these.
    pub lts: LTS,
    // TODO Option is weird -- we should upfront filter this out.
    pub cost: Option<(usize, usize)>,
    // TODO Maybe generalize as a cost and a bunch of properties per edge -- like proximity
    // modifiers for greenspace, lighting, commercial areas
    pub nearby_amenities: usize,
}

impl Edge {
    pub fn apply_elevation<R: Read + Seek + Send>(&self, elevation_data: &mut GeoTiffElevation<R>) -> Option<(f64, (f64, f64))> {
        let slope = if let Some(slope) = self.get_slope(elevation_data){
            slope
        } else {
            return None
        };

        let length = self.length_meters;
        
        let forward_slope_factor = Edge::calculate_slope_factor(slope, length);
        let backward_slope_factor = Edge::calculate_slope_factor(-slope, length);

        Some((slope, (forward_slope_factor, backward_slope_factor)))
    }
    
    fn calculate_slope_factor(slope: f64, length: f64) -> f64 {
    
        let g = match (slope, length) {
            (x,y) if 13.0 >= x && x > 10.0 && y > 15.0 => {
                4.0
            },
            (x,y) if x < 8.0 && x <= 10.0 && y > 30.0 => {
                4.5
            },
            (x,y) if x < 5.0 && x <= 8.0 && y > 60.0 => {
                5.0
            },
            (x,y) if x < 3.0 && x <= 5.0 && y > 120.0=> {
                6.0
            },
            _ => {
                7.0
            }
        };

        let slope_factor = match slope {
            x if x < -30.0 => { 1.5 },
            x if x < 0.0 && x >= -30.0 => { 
                1.0 + 2.0*0.7*slope/13.0 + 0.7 * slope * slope /13.0 /13.0
            },
            x if x <= 20.0 && x >= 0.0 => {
                1.0 + slope * slope / g / g
            },
            _ => { 10.0 }
        };
        
        slope_factor
    }  

    fn get_slope<R: Read + Seek + Send>(&self, elevation_data: &mut GeoTiffElevation<R>) -> Option<f64> {
        let first_node = self.geometry[0];
        let second_node = self.geometry[1];
        
        let first_node_height = match elevation_data.get_height_for_lon_lat(first_node.lat as f32/1e7, first_node.lon as f32/1e7) {
            Some(elevation) => elevation,
            None => return None
        };
        
        let second_node_height = match elevation_data.get_height_for_lon_lat(second_node.lat as f32/1e7, second_node.lon as f32/1e7) {
            Some(elevation) => elevation,
            None => return None
        };
        
        let slope = (second_node_height -  first_node_height) / self.length_meters as f32 * 100.0;
        Some(slope.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Edge;

    #[test]
    fn speed_slope_test() {
        let speed_flat = 15.0;
        let slope = 3.0;
        let length = 50.0;
        let slope_factor = Edge::calculate_slope_factor(slope, length);
        let slope_speed = speed_flat/slope_factor;
        let delta = slope_speed - 12.67241;
        assert!(delta < f64::EPSILON);

        let slope = -8.0;
        let length = 100.0;
        let slope_factor = Edge::calculate_slope_factor(slope, length);
        let slope_speed = speed_flat/slope_factor;
        let delta = slope_speed - 37.17009;
        assert!(delta < f64::EPSILON);


    }
}


