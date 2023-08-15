use std::collections::HashMap;
use std::time::Instant;

use anyhow::Result;
use geo::{BoundingRect, Contains, MultiPolygon};
use geojson::{GeoJson, Value};
use indicatif::HumanCount;
use nanorand::{Rng, WyRand};
use rstar::{RTree, AABB};
use serde::Deserialize;

use super::input::ODPattern;
use super::requests::Request;

pub fn generate(
    pattern: ODPattern,
    origins_path: &str,
    destinations_path: &str,
    rng_seed: u64,
) -> Result<Vec<Request>> {
    let mut start = Instant::now();
    println!("Loading origins from {origins_path}");
    let origins = load_points(origins_path)?;
    println!(
        "That took {:?}. Loading destinations from {destinations_path}",
        Instant::now().duration_since(start)
    );
    start = Instant::now();
    let destinations = load_points(destinations_path)?;
    println!("That took {:?}", Instant::now().duration_since(start));
    println!(
        "Got {} origins and {} destination",
        HumanCount(origins.len() as u64),
        HumanCount(destinations.len() as u64)
    );

    let mut requests = Vec::new();
    match pattern {
        ODPattern::FromEveryOriginToOneDestination => {
            for pt in origins {
                requests.push(Request {
                    x1: pt.0,
                    y1: pt.1,
                    x2: destinations[0].0,
                    y2: destinations[0].1,
                });
            }
        }
        ODPattern::FromEveryOriginToNearestDestination => {
            let closest = RTree::bulk_load(destinations);
            for pt in origins {
                let goto = closest.nearest_neighbor(&pt).unwrap();
                requests.push(Request {
                    x1: pt.0,
                    y1: pt.1,
                    x2: goto.0,
                    y2: goto.1,
                });
            }
        }
        ODPattern::BetweenZones {
            zones_path,
            csv_path,
        } => {
            start = Instant::now();
            println!("Loading zones from {zones_path}");
            let zones = load_zones(&zones_path)?;
            println!(
                "That took {:?}. Matching points to zones",
                Instant::now().duration_since(start)
            );
            start = Instant::now();
            let origins_per_zone = points_per_polygon(origins, &zones);
            let destinations_per_zone = points_per_polygon(destinations, &zones);
            println!(
                "That took {:?}. Generating requests from {csv_path}",
                Instant::now().duration_since(start)
            );

            let mut rng = WyRand::new_seed(rng_seed);

            for rec in csv::Reader::from_reader(fs_err::File::open(csv_path)?).deserialize() {
                let row: BetweenZonesRow = rec?;
                for _ in 0..row.count {
                    let from = match origins_per_zone.get(&row.from) {
                        Some(points) => {
                            if points.is_empty() {
                                bail!("Zone {} has no origin points", row.from);
                            }
                            points[rng.generate_range(0..points.len())]
                        }
                        None => {
                            bail!("Unknown zone {}", row.from);
                        }
                    };
                    let to = match destinations_per_zone.get(&row.to) {
                        Some(points) => {
                            if points.is_empty() {
                                bail!("Zone {} has no destination points", row.to);
                            }
                            points[rng.generate_range(0..points.len())]
                        }
                        None => {
                            bail!("Unknown zone {}", row.to);
                        }
                    };
                    requests.push(Request {
                        x1: from.0,
                        y1: from.1,
                        x2: to.0,
                        y2: to.1,
                    });
                }
            }
        }
    }

    Ok(requests)
}

// TODO Use geo?
fn load_points(path: &str) -> Result<Vec<(f64, f64)>> {
    let gj = fs_err::read_to_string(path)?.parse::<GeoJson>()?;
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

fn points_per_polygon(
    points: Vec<(f64, f64)>,
    polygons: &HashMap<String, MultiPolygon<f64>>,
) -> HashMap<String, Vec<(f64, f64)>> {
    let tree = RTree::bulk_load(points);

    let mut output = HashMap::new();
    for (key, polygon) in polygons {
        let mut pts_inside = Vec::new();
        let bounds = polygon.bounding_rect().unwrap();
        let min = bounds.min();
        let max = bounds.max();
        let envelope: AABB<(f64, f64)> = AABB::from_corners((min.x, min.y), (max.x, max.y));
        for pt in tree.locate_in_envelope(&envelope) {
            if polygon.contains(&geo::Point::new(pt.0, pt.1)) {
                pts_inside.push(*pt);
            }
        }
        output.insert(key.clone(), pts_inside);
    }

    // TODO Check every zone has points, to fail-fast
    output
}

// TODO Can we use this?
/*#[derive(Deserialize)]
struct Zone {
    #[serde(deserialize_with = "deserialize_geometry")]
    geometry: geo_types::MultiPolygon<f64>,
    name: String,
}*/

/// Extract multipolygon zones from a GeoJSON file, using the "name" property as the key in the
/// resulting map.
fn load_zones(geojson_path: &str) -> Result<HashMap<String, MultiPolygon<f64>>> {
    let geojson_input = fs_err::read_to_string(geojson_path)?;
    let geojson = geojson_input.parse::<GeoJson>()?;

    let mut zones: HashMap<String, MultiPolygon<f64>> = HashMap::new();
    if let GeoJson::FeatureCollection(collection) = geojson {
        for feature in collection.features {
            if let Some(zone_name) = feature
                .property("name")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string())
            {
                let gj_geom: geojson::Geometry = feature.geometry.unwrap();
                let geo_geometry: geo::Geometry<f64> = gj_geom.try_into().unwrap();
                if let geo::Geometry::MultiPolygon(mp) = geo_geometry {
                    zones.insert(zone_name, mp);
                } else if let geo::Geometry::Polygon(p) = geo_geometry {
                    zones.insert(zone_name, p.into());
                } else {
                    bail!("Feature has geometry other than a Polygon or MultiPolygon");
                }
            } else {
                bail!("Feature doesn't have a string zone \"name\": {:?}", feature);
            }
        }
    }
    Ok(zones)
}

#[derive(Deserialize)]
struct BetweenZonesRow {
    from: String,
    to: String,
    count: usize,
}
