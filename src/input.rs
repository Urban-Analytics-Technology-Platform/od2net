use serde::{Deserialize, Serialize};

/// Everything needed to run the pipeline
#[derive(Serialize, Deserialize)]
pub struct InputConfig {
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
    Generate {
        pattern: ODPattern,
        /// Path to a GeoJSON file with origin points to use
        origins_path: String,
        /// Path to a GeoJSON file with destination points to use
        destinations_path: String,
    },
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
    Custom,
}
