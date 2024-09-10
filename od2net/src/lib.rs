#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

// TODO Restructure
pub mod config;
pub mod detailed_route_output;
pub mod network;
pub mod node_map;
pub mod od;
pub mod plugins;
pub mod requests;
pub mod router;
pub mod timer;
pub mod utils;

use indicatif::HumanCount;
use instant::Duration;
use serde::Serialize;

use lts::LTS;

// TODO Move, maybe an output.rs with big chunks of network too
#[derive(Serialize)]
pub struct OutputMetadata {
    pub config: config::InputConfig,
    pub num_origins: usize,
    pub num_destinations: usize,
    pub num_requests: usize,
    pub num_succeeded_requests: usize,
    pub num_failed_requests_same_endpoints: usize,
    pub num_failed_requests_no_path: usize,
    pub num_edges_with_count: usize,
    pub routing_time_seconds: f32,
    pub total_meters_not_allowed: f64,
    pub total_meters_lts1: f64,
    pub total_meters_lts2: f64,
    pub total_meters_lts3: f64,
    pub total_meters_lts4: f64,
    // These two aren't recorded in the GeoJSON or PMTiles output, because we'd have to go back and
    // update the files!
    pub total_time_seconds: Option<f32>,
    pub tippecanoe_time_seconds: Option<f32>,
}

impl OutputMetadata {
    pub fn new(
        config: config::InputConfig,
        counts: &network::Counts,
        num_requests: usize,
        routing_time: Duration,
    ) -> Self {
        Self {
            config,
            num_origins: counts.count_per_origin.len(),
            num_destinations: counts.count_per_destination.len(),
            num_requests,
            num_succeeded_requests: num_requests - counts.num_errors(),
            num_failed_requests_same_endpoints: counts.errors_same_endpoints.len(),
            num_failed_requests_no_path: counts.errors_no_path.len(),
            num_edges_with_count: counts.count_per_edge.len(),
            routing_time_seconds: routing_time.as_secs_f32(),
            total_time_seconds: None,
            tippecanoe_time_seconds: None,
            total_meters_not_allowed: counts.total_distance_by_lts[LTS::NotAllowed as u8 as usize],
            total_meters_lts1: counts.total_distance_by_lts[LTS::LTS1 as u8 as usize],
            total_meters_lts2: counts.total_distance_by_lts[LTS::LTS2 as u8 as usize],
            total_meters_lts3: counts.total_distance_by_lts[LTS::LTS3 as u8 as usize],
            total_meters_lts4: counts.total_distance_by_lts[LTS::LTS4 as u8 as usize],
        }
    }

    pub fn describe(&self) {
        println!("Input: {}", self.config.requests.description);
        for (label, count) in [
            ("Origins", self.num_origins),
            ("Destinations", self.num_destinations),
            ("Requests", self.num_requests),
            ("Requests (succeeded)", self.num_succeeded_requests),
            (
                "Requests (failed because same endpoints)",
                self.num_failed_requests_same_endpoints,
            ),
            (
                "Requests (failed because no path)",
                self.num_failed_requests_no_path,
            ),
            ("Edges with a count", self.num_edges_with_count),
        ] {
            println!("- {label}: {}", HumanCount(count as u64));
        }
        for (label, meters) in [
            // For bugspotting
            ("not allowed roads", self.total_meters_not_allowed),
            ("LTS 1 roads", self.total_meters_lts1),
            ("LTS 2 roads", self.total_meters_lts2),
            ("LTS 3 roads", self.total_meters_lts3),
            ("LTS 4 roads", self.total_meters_lts4),
        ] {
            let km = meters / 1000.0;
            println!("- Total distance on {label}: {km:.1} km");
        }
    }
}
