use std::collections::HashMap;
use std::io::BufReader;

use anyhow::Result;
use fs_err::File;
use geo::{BoundingRect, Centroid, Contains, MultiPolygon, Point};
use geojson::{FeatureReader, Value};
use indicatif::HumanCount;
use rand::{prelude::SliceRandom, rngs::StdRng, SeedableRng};
use rstar::{PointDistance, RTree, RTreeObject, AABB};
use serde::Deserialize;

use super::config::{ODPattern, Requests};
use super::network::Network;
use super::requests::Request;
use super::timer::Timer;

pub fn generate_requests(
    config: &Requests,
    input_directory: String,
    network: &Network,
    rng_seed: u64,
    timer: &mut Timer,
) -> Result<Vec<Request>> {
    let origins = if config.origins_path.is_empty() {
        Vec::new()
    } else {
        timer.start("Loading origins");
        let origins = load_points(format!("{input_directory}/{}", config.origins_path))?;
        timer.stop();
        origins
    };
    let destinations = if config.destinations_path.is_empty() {
        Vec::new()
    } else {
        timer.start("Loading destinations");
        let destinations = load_points(format!("{input_directory}/{}", config.destinations_path))?;
        timer.stop();
        destinations
    };
    println!(
        "Got {} origins and {} destination",
        HumanCount(origins.len() as u64),
        HumanCount(destinations.len() as u64)
    );

    let mut requests = Vec::new();
    match &config.pattern {
        ODPattern::FromEveryOriginToOneDestination => {
            timer.start(format!(
                "FromEveryOriginToOneDestination for {} origins",
                HumanCount(origins.len() as u64),
            ));
            for pt in origins {
                requests.push(Request {
                    x1: pt.lon,
                    y1: pt.lat,
                    x2: destinations[0].lon,
                    y2: destinations[0].lat,
                    origin: None,
                    destination: None,
                });
            }
            timer.stop();
        }
        ODPattern::FromEveryOriginToNearestDestination => {
            timer.start("Prep rtree for destinations");
            let closest = RTree::bulk_load(destinations);
            timer.stop();
            timer.start(format!(
                "FromEveryOriginToNearestDestination for {} origins",
                HumanCount(origins.len() as u64),
            ));
            for pt in origins {
                let goto = closest.nearest_neighbor(&(pt.lon, pt.lat)).unwrap();
                requests.push(Request {
                    x1: pt.lon,
                    y1: pt.lat,
                    x2: goto.lon,
                    y2: goto.lat,
                    origin: None,
                    destination: None,
                });
            }
            timer.stop();
        }
        ODPattern::BetweenZones {
            zones_path,
            csv_path,
            origin_zone_centroid_fallback,
            destination_zone_centroid_fallback,
        } => {
            let zones_path = format!("{input_directory}/{zones_path}");
            let csv_path = format!("{input_directory}/{csv_path}");

            timer.start(format!("Loading zones from {zones_path}"));
            let zones = load_zones(&zones_path)?;
            timer.stop();
            timer.start("Matching points to zones");
            let origins_per_zone =
                points_per_polygon("origin", origins, &zones, *origin_zone_centroid_fallback)?;
            let destinations_per_zone = points_per_polygon(
                "destination",
                destinations,
                &zones,
                *destination_zone_centroid_fallback,
            )?;
            timer.stop();

            timer.start(format!("Generating requests from {csv_path}"));
            let mut rng = StdRng::seed_from_u64(rng_seed);

            for rec in csv::Reader::from_reader(File::open(csv_path)?).deserialize() {
                let row: BetweenZonesRow = rec?;
                let Some(from_points) = origins_per_zone.get(&row.from) else {
                    bail!("Unknown zone {}", row.from);
                };
                let Some(to_points) = destinations_per_zone.get(&row.to) else {
                    bail!("Unknown zone {}", row.to);
                };
                for _ in 0..row.count {
                    // TODO choose_weighted is O(n); there are alternatives if this ever becomes a
                    // problem.
                    let from = from_points
                        .choose_weighted(&mut rng, |pt| pt.weight)
                        .unwrap();
                    let to = to_points.choose_weighted(&mut rng, |pt| pt.weight).unwrap();
                    requests.push(Request {
                        x1: from.lon,
                        y1: from.lat,
                        x2: to.lon,
                        y2: to.lat,
                        origin: Some(row.from.clone()),
                        destination: Some(row.to.clone()),
                    });
                }
            }
            timer.stop();
        }
        // TODO Maybe refactor these -- allow zones to be empty, O and D can have named points
        ODPattern::ZoneToPoint {
            zones_path,
            csv_path,
            destinations_path,
            origin_zone_centroid_fallback,
        } => {
            let zones_path = format!("{input_directory}/{zones_path}");
            let csv_path = format!("{input_directory}/{csv_path}");
            let destinations_path = format!("{input_directory}/{destinations_path}");

            timer.start(format!(
                "Loading zones from {zones_path} and named destinations from {destinations_path}"
            ));
            let zones = load_zones(&zones_path)?;
            let destinations = load_named_points(&destinations_path)?;
            timer.stop();
            timer.start("Matching points to zones");
            let origins_per_zone =
                points_per_polygon("origin", origins, &zones, *origin_zone_centroid_fallback)?;
            timer.stop();

            timer.start(format!("Generating requests from {csv_path}"));
            let mut rng = StdRng::seed_from_u64(rng_seed);

            for rec in csv::Reader::from_reader(File::open(csv_path)?).deserialize() {
                let row: BetweenZonesRow = rec?;
                let Some(from_points) = origins_per_zone.get(&row.from) else {
                    bail!("Unknown zone {}", row.from);
                };
                let Some(to) = destinations.get(&row.to) else {
                    bail!("Unknown destination {}", row.to);
                };
                for _ in 0..row.count {
                    let from = from_points
                        .choose_weighted(&mut rng, |pt| pt.weight)
                        .unwrap();
                    requests.push(Request {
                        x1: from.lon,
                        y1: from.lat,
                        x2: to.0,
                        y2: to.1,
                        origin: Some(row.from.clone()),
                        destination: Some(row.to.clone()),
                    });
                }
            }
            timer.stop();
        }
        ODPattern::LineStrings(path) => {
            timer.start(format!("Loading LineString requests from {path}"));
            requests = Request::load_from_geojson(format!("{input_directory}/{path}"))?;
            timer.stop();
        }
        ODPattern::AllPairsIntersections => {
            for from in network.intersections.values() {
                let (x1, y1) = from.to_degrees();
                for to in network.intersections.values() {
                    let (x2, y2) = to.to_degrees();
                    requests.push(Request {
                        x1,
                        y1,
                        x2,
                        y2,
                        origin: None,
                        destination: None,
                    });
                }
            }
        }
    }

    Ok(requests)
}

/// A point with an associated relative weight. Higher weights are more likely to be sampled.
#[derive(Clone, Copy)]
struct WeightedPoint {
    // TODO Use geo? Could maybe just read in one big batch and have more validation
    lon: f64,
    lat: f64,
    weight: f64,
}

// TODO Is GeomWithData simpler?
impl RTreeObject for WeightedPoint {
    type Envelope = AABB<(f64, f64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.lon, self.lat))
    }
}

impl PointDistance for WeightedPoint {
    fn distance_2(&self, point: &(f64, f64)) -> f64 {
        // Use Euclidean distance on WGS84
        (self.lon - point.0).powi(2) + (self.lat - point.1).powi(2)
    }
}

fn load_points(path: String) -> Result<Vec<WeightedPoint>> {
    println!("Loading points from {path}");
    let reader = FeatureReader::from_reader(BufReader::new(File::open(path)?));
    let mut points = Vec::new();
    for feature in reader.features() {
        let feature = feature?;
        if let Some(ref geometry) = feature.geometry {
            if let Value::Point(pt) = &geometry.value {
                let weight = feature
                    .property("weight")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(1.0);
                points.push(WeightedPoint {
                    lon: pt[0],
                    lat: pt[1],
                    weight,
                });
            }
        }
    }
    Ok(points)
}

// TODO Refactor?
fn load_named_points(path: &str) -> Result<HashMap<String, (f64, f64)>> {
    let reader = FeatureReader::from_reader(BufReader::new(File::open(path)?));
    let mut result = HashMap::new();
    for feature in reader.features() {
        let feature = feature?;
        if let Some(name) = feature
            .property("name")
            .and_then(|x| x.as_str())
            .map(|x| x.to_string())
        {
            if let Some(geometry) = feature.geometry {
                if let Value::Point(pt) = geometry.value {
                    result.insert(name, (pt[0], pt[1]));
                }
            }
        } else {
            bail!(
                "A feature in {path} doesn't have a string \"name\". Properties: {:?}",
                feature.properties
            );
        }
    }
    Ok(result)
}

fn points_per_polygon(
    name: &str,
    points: Vec<WeightedPoint>,
    polygons: &HashMap<String, MultiPolygon>,
    use_centroids_for_empty_zones: bool,
) -> Result<HashMap<String, Vec<WeightedPoint>>> {
    let tree = RTree::bulk_load(points);

    let mut empty = Vec::new();
    let mut output = HashMap::new();
    for (key, polygon) in polygons {
        let mut pts_inside = Vec::new();
        let bounds = polygon.bounding_rect().unwrap();
        let min = bounds.min();
        let max = bounds.max();
        let envelope: AABB<(f64, f64)> = AABB::from_corners((min.x, min.y), (max.x, max.y));
        for pt in tree.locate_in_envelope(&envelope) {
            if polygon.contains(&Point::new(pt.lon, pt.lat)) {
                pts_inside.push(*pt);
            }
        }
        if pts_inside.is_empty() {
            empty.push(key);
        }
        output.insert(key.clone(), pts_inside);
    }

    if !empty.is_empty() && !use_centroids_for_empty_zones {
        bail!("Some zones have no matching {name} points: {:?}", empty);
    }
    println!(
        "{} zones have no matching {name} points. Using centroid instead.",
        HumanCount(empty.len() as u64)
    );
    for key in empty {
        if let Some(centroid) = polygons[key].centroid() {
            output.insert(
                key.clone(),
                vec![WeightedPoint {
                    lon: centroid.x(),
                    lat: centroid.y(),
                    weight: 1.0,
                }],
            );
        } else {
            bail!("{key} had no matching {name} points, and couldn't calculate its centroid");
        }
    }

    Ok(output)
}

/// Extract multipolygon zones from a GeoJSON file, using the "name" property as the key in the
/// resulting map.
fn load_zones(geojson_path: &str) -> Result<HashMap<String, MultiPolygon>> {
    let reader = FeatureReader::from_reader(BufReader::new(File::open(geojson_path)?));

    let mut zones: HashMap<String, MultiPolygon> = HashMap::new();
    for feature in reader.features() {
        let feature = feature?;
        if let Some(zone_name) = feature
            .property("name")
            .and_then(|x| x.as_str())
            .map(|x| x.to_string())
        {
            let gj_geom: geojson::Geometry = feature.geometry.unwrap();
            let geo_geometry: geo::Geometry = gj_geom.try_into().unwrap();
            if let geo::Geometry::MultiPolygon(mp) = geo_geometry {
                zones.insert(zone_name, mp);
            } else if let geo::Geometry::Polygon(p) = geo_geometry {
                zones.insert(zone_name, p.into());
            } else {
                bail!(
                    "A feature in {geojson_path} has geometry other than a Polygon or MultiPolygon"
                );
            }
        } else {
            bail!(
                "A feature in {geojson_path} doesn't have a string \"name\". Properties: {:?}",
                feature.properties
            );
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
