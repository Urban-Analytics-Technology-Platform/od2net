use anyhow::Result;
use geojson::{GeoJson, Value};
use indicatif::HumanCount;

use super::requests::Request;

pub fn generate(origins_path: &str, destinations_path: &str) -> Result<Vec<Request>> {
    let origins = load_subpoints(origins_path)?;
    let destinations = load_subpoints(destinations_path)?;
    println!(
        "Got {} origins and {} destination",
        HumanCount(origins.len() as u64),
        HumanCount(destinations.len() as u64)
    );

    // Just a quick pattern: from every origin to one destination
    let mut requests = Vec::new();
    for pt in origins {
        requests.push(Request {
            x1: pt.0,
            y1: pt.1,
            x2: destinations[0].0,
            y2: destinations[0].1,
        });
    }

    Ok(requests)
}

fn load_subpoints(path: &str) -> Result<Vec<(f64, f64)>> {
    let gj = std::fs::read_to_string(path)?.parse::<GeoJson>()?;
    let mut points = Vec::new();
    if let GeoJson::FeatureCollection(collection) = gj {
        for feature in collection.features {
            if let Some(geometry) = feature.geometry {
                if let Value::Point(pt) = geometry.value {
                    points.push((pt[0], pt[1]));
                }
            }
        }
    }
    Ok(points)
}
