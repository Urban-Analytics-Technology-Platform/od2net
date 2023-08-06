use serde::{Deserialize, Serialize};

/// Everything needed to run the pipeline
#[derive(Serialize, Deserialize)]
pub struct InputConfig {
    /// Path to a directory containing:
    ///
    /// - input.osm.pbf
    /// - Optionally, origins.geojson and destinations.geojson
    ///
    /// Cached and output files will get created in here by this pipeline:
    ///
    /// - network.bin
    /// - ch.bin
    /// - output.geojson
    pub directory: String,

    pub requests: Requests,

    pub routing: Routing,
}

#[derive(Serialize, Deserialize)]
pub enum Requests {
    Odjitter {
        /// A GeoJSON file with LineString requests
        path: String,
        /// A percent (0 to 1000 -- note NOT 100) of requests to use. Defaults to all of them.
        sample_requests: Option<usize>,
        /// Cap requests to exactly this many.
        cap_requests: Option<usize>,
    },
    /// These all assume <directory>/origins.geojson and <directory>/destinations.geojson exist
    Generate { pattern: ODPattern },
}

#[derive(Serialize, Deserialize)]
pub enum ODPattern {
    /// One trip from every origin to just the first destination
    FromEveryOriginToOneDestination,
    /// One trip from every origin to the closest (as the crow flies) destination
    FromEveryOriginToNearestDestination,
}

#[derive(Serialize, Deserialize)]
pub enum Routing {
    OSRM {
        /// How many requests to OSRM to have in-flight at once. Defaults to 10.
        concurrency: Option<usize>,
    },
    FastPaths {
        cost: CostFunction,
    },
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum CostFunction {
    /// Just find the shortest distance path
    Distance,
    /// Heavily penalize main roads
    AvoidMainRoads,
}
