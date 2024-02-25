mod amenities;
mod create_from_osm;
mod greenspace;
mod output;

use std::collections::HashMap;
use std::io::BufReader;
use std::io::{Read, Seek};

use anyhow::Result;
use elevation::GeoTiffElevation;
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
    // A 3% slope is represented as 3.0.
    pub slope: Option<f64>,
    // A factor to multiply cost by in the (forwards, backwards) direction
    pub slope_factor: Option<(f64, f64)>,
    // Storing the derived field is negligible for file size
    pub length_meters: f64,
    // LTS is often incorporated in cost, but is also used for visualization. It's useful to
    // conceptually separate these.
    pub lts: LTS,
    // TODO Option is weird -- we should upfront filter this out.
    pub forward_cost: Option<usize>,
    pub backward_cost: Option<usize>,
    // TODO Maybe generalize as a cost and a bunch of properties per edge -- like proximity
    // modifiers for greenspace, lighting, commercial areas
    pub nearby_amenities: usize,
}

impl Edge {
    /// Sets `slope` and `slope_factor` if true. If false, failed to get data.
    pub fn apply_elevation<R: Read + Seek + Send>(
        &mut self,
        geotiff: &mut GeoTiffElevation<R>,
    ) -> bool {
        let Some(slope) = self.get_slope(geotiff) else {
            return false;
        };
        self.slope = Some(slope);
        self.slope_factor = Some((
            calculate_slope_factor(slope, self.length_meters),
            calculate_slope_factor(-slope, self.length_meters),
        ));
        true
    }

    fn get_slope<R: Read + Seek + Send>(&self, geotiff: &mut GeoTiffElevation<R>) -> Option<f64> {
        let (lon1, lat1) = self.geometry[0].to_degrees();
        let (lon2, lat2) = self.geometry.last().unwrap().to_degrees();

        let height1 = geotiff.get_height_for_lon_lat(lon1 as f32, lat1 as f32)?;
        let height2 = geotiff.get_height_for_lon_lat(lon2 as f32, lat2 as f32)?;

        let slope = (height2 - height1) / (self.length_meters as f32) * 100.0;
        Some(slope.into())
    }
}

/// This returns a factor to multiply cost by, to adjust the speed of a cyclist. See
/// <https://github.com/U-Shift/Declives-RedeViaria/blob/main/SpeedSlopeFactor/SpeedSlopeFactor.md#speed-slope-factor-1>.
fn calculate_slope_factor(slope: f64, length: f64) -> f64 {
    // Ported from https://github.com/U-Shift/Declives-RedeViaria/blob/5b5680ba769ab57f0fe061fd16c626cec66a0452/SpeedSlopeFactor/SpeedSlopeFactor.Rmd#L114
    let g = if slope > 3.0 && slope <= 5.0 && length > 120.0 {
        6.0
    } else if slope > 5.0 && slope <= 8.0 && length > 60.0 {
        5.0
    } else if slope > 8.0 && slope <= 10.0 && length > 30.0 {
        4.5
    } else if slope > 10.0 && slope <= 13.0 && length > 15.0 {
        4.0
    } else {
        7.0
    };

    // TODO Check this one again
    let slope_factor = if slope < -30.0 {
        1.5
    } else if slope < 0.0 && slope >= -30.0 {
        1.0 + 2.0 * 0.7 * slope / 13.0 + 0.7 * slope * slope / 13.0 / 13.0
    } else if slope <= 20.0 && slope >= 0.0 {
        1.0 + slope * slope / g / g
    } else {
        10.0
    };

    slope_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speed_slope_test() {
        let speed_flat = 15.0;
        let slope = 3.0;
        let length = 50.0;
        let slope_factor = calculate_slope_factor(slope, length);
        let slope_speed = speed_flat / slope_factor;
        let delta = slope_speed - 12.67241;
        assert!(delta < 0.00001);

        let slope = -8.0;
        let length = 100.0;
        let slope_factor = calculate_slope_factor(slope, length);
        let slope_speed = speed_flat / slope_factor;
        let delta = slope_speed - 37.17009;
        assert!(delta < 0.00001);
    }
}
