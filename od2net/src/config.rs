use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Everything needed to run the pipeline.
///
/// All paths are relative to the `input/` directory.
#[derive(Serialize, Deserialize)]
pub struct InputConfig {
    pub requests: Requests,

    pub cost: CostFunction,

    pub uptake: Uptake,

    pub lts: LtsMapping,

    pub dem: Option<String>, 
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Requests {
    pub description: String,
    pub pattern: ODPattern,
    // TODO These are irrelevant for some cases
    /// Path to a GeoJSON file with points to use as origins
    pub origins_path: String,
    /// Path to a GeoJSON file with points to use as destinations
    pub destinations_path: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ODPattern {
    /// One trip from every origin to just the first destination
    FromEveryOriginToOneDestination,
    /// One trip from every origin to the closest (as the crow flies) destination
    FromEveryOriginToNearestDestination,
    /// Trips between named zones
    BetweenZones {
        /// Path to a GeoJSON file containing Polygons and MultiPolygons with a "name" property
        zones_path: String,
        /// Path to a CSV file that must have 3 columns "from", "to", and "count". The first
        /// two must match zone names. "count" must be an integer.
        csv_path: String,
    },
    ZoneToPoint {
        /// Path to a GeoJSON file containing Polygons and MultiPolygons with a "name" property
        zones_path: String,
        /// Path to a CSV file that must have 3 columns "from", "to", and "count". The first
        /// two must match zone and destination names. "count" must be an integer.
        csv_path: String,
        /// Path to a GeoJSON file containing Points with a "name" property
        destinations_path: String,
        /// If a zone doesn't have any matching origin points, use the zone's centroid instead.
        origin_zone_centroid_fallback: bool,
    },
    /// Just read GeoJSON LineStrings from this path
    LineStrings(String),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum CostFunction {
    /// Just find the most direct path, minimizing distance. This is equivalent to ByLTS with all
    /// weights set to 1.
    Distance,
    /// Multiply distance by a factor for each LTS classification
    ByLTS {
        lts1: f64,
        lts2: f64,
        lts3: f64,
        lts4: f64,
    },
    Generalized(GeneralizedCostFunction),
    /// Multiply distance by a factor based on the OSM highway tag. If the type isn't present, it
    /// won't be allowed at all.
    OsmHighwayType(HashMap<String, f64>),
    /// Run this command to calculate edge cost. STDIN will contain a JSON array of objects, each
    /// with OSM tags representing one segment and extra properties (length_meters,
    /// nearby_amenities, lts). The output must be an equally sized JSON array of integers,
    /// representing the cost for that edge.
    ExternalCommand(String),
}

impl CostFunction {
    pub fn normalize(&mut self) -> Result<()> {
        if let CostFunction::Generalized(ref mut params) = self {
            params.normalize()?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralizedCostFunction {
    // These 3 control overall tradeoffs. They must sum to 1.0.
    pub tradeoff_lts: f64,
    pub tradeoff_amenities: f64,
    pub tradeoff_greenspace: f64,

    // Weights between 0 (good) and 1 (bad) for each LTS category
    pub lts1: f64,
    pub lts2: f64,
    pub lts3: f64,
    pub lts4: f64,

    // The weight for nearby amenities will be 1 (bad) if the road has less than this number, or 0
    // (good) if it has at least this many.
    pub minimum_amenities: usize,
}

impl GeneralizedCostFunction {
    pub fn normalize(&mut self) -> Result<()> {
        // Normalize the tradeoffs to sum to 1
        let sum = self.tradeoff_lts + self.tradeoff_amenities + self.tradeoff_greenspace;
        self.tradeoff_lts /= sum;
        self.tradeoff_amenities /= sum;
        self.tradeoff_greenspace /= sum;

        // Check the LTS weights are in the correct range
        for x in [self.lts1, self.lts2, self.lts3, self.lts4] {
            if x < 0.0 || x > 1.0 {
                bail!("A LTS weight is {x}, but it needs to be in [0, 1]");
            }
        }

        Ok(())
    }
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

#[derive(Serialize, Deserialize)]
pub enum LtsMapping {
    SpeedLimitOnly,
    BikeOttawa,
    /// Run this command to calculate LTS. STDIN will contain a JSON array of objects, each with
    /// OSM tags representing one segment. The output must be an equally sized JSON array of
    /// numbers 0-4, representing the resulting LTS.
    ExternalCommand(String),
}

