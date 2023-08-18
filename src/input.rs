use serde::{Deserialize, Serialize};

/// Everything needed to run the pipeline
#[derive(Serialize, Deserialize)]
pub struct InputConfig {
    pub requests: Requests,

    pub routing: Routing,

    pub uptake: Uptake,
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
        /// Defaults to <directory>/input/origins.geojson
        origins_path: Option<String>,
        /// Defaults to <directory>/input/destinations.geojson
        destinations_path: Option<String>,
    },
}

#[derive(Serialize, Deserialize)]
pub enum ODPattern {
    /// One trip from every origin to just the first destination
    FromEveryOriginToOneDestination,
    /// One trip from every origin to the closest (as the crow flies) destination
    FromEveryOriginToNearestDestination,
    /// Trips between named zones
    BetweenZones {
        // TODO Maybe use default filenames here too
        /// Path to a GeoJSON file containing Polygons and MultiPolygons with a "name" property
        zones_path: String,
        /// Path to a CSV file that must have 3 columns "from", "to", and "count". The first
        /// two must match zone names. "count" must be an integer.
        csv_path: String,
    },
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

#[derive(Serialize, Deserialize)]
pub enum Uptake {
    /// Don't do anything -- every route counts for 1
    Identity,
    /// 0 for trips greater than this distance, 1 otherwise
    CutoffMaxDistanceMeters(f64),
    /// Defined by https://github.com/ITSLeeds/pct/blob/HEAD/R/uptake.R
    GovTargetPCT,
    /// Defined by https://github.com/ITSLeeds/pct/blob/HEAD/R/uptake.R
    GoDutchPCT,
}
