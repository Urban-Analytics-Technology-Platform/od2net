use geo::Centroid;
use geo_types::GeometryCollection;
use geojson::{quick_collection, FeatureCollection, GeoJson};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Give a .geojson as input");
    }

    let mut start = Instant::now();
    println!("Read to string");
    let raw_string = std::fs::read_to_string(&args[1]).unwrap();
    println!("Parse");
    let gj = raw_string.parse::<GeoJson>().unwrap();
    println!("To geo world");
    let collection: GeometryCollection = quick_collection(&gj).unwrap();
    println!(
        "Read {} things, took {:?}",
        collection.len(),
        Instant::now().duration_since(start)
    );

    start = Instant::now();
    println!("Transforming to centroid");
    let mut points = Vec::new();
    for input in collection {
        let pt = input.centroid().unwrap();
        points.push(pt);
    }
    println!("Writing");
    let fc = FeatureCollection::from(&GeometryCollection::from(points));
    std::fs::write("centroids.geojson", serde_json::to_string(&fc).unwrap()).unwrap();
    println!("That took {:?}", Instant::now().duration_since(start));
}
