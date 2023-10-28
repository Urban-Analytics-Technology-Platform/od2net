use std::f64;

use geojson::{Feature, Value};

// TODO Everything in here can hopefully be replaced with a crate

// WGS84 to Mercator
pub fn wgs84_to_web_mercator(c: [f64; 2]) -> [f64; 2] {
    static A: f64 = 6378137.0;
    static MAXEXTENT: f64 = 20037508.342789244;
    static D2R: f64 = f64::consts::PI / 180.0;

    [
        (A * c[0] * D2R).max(-MAXEXTENT).min(MAXEXTENT) as f64,
        (A * (((f64::consts::PI * 0.25f64) + (0.5f64 * c[1] * D2R)).tan()).ln())
            .max(-MAXEXTENT)
            .min(MAXEXTENT) as f64,
    ]
}

pub fn calculate_bbox(features: &Vec<Feature>) -> (f64, f64, f64, f64) {
    // TODO Convert to geo and just use something there?
    let mut min_lon = f64::MAX;
    let mut max_lon = f64::MIN;
    let mut min_lat = f64::MAX;
    let mut max_lat = f64::MIN;

    for f in features {
        if let Some(ref geometry) = f.geometry {
            if let Value::LineString(ref line_string) = geometry.value {
                for pt in line_string {
                    min_lon = min_lon.min(pt[0]);
                    min_lat = min_lat.min(pt[1]);
                    max_lon = max_lon.max(pt[0]);
                    max_lat = max_lat.max(pt[1]);
                }
            }
        }
    }

    (min_lon, min_lat, max_lon, max_lat)
}

pub fn bbox_to_tiles(bbox: (f64, f64, f64, f64), zoom: u32) -> (u32, u32, u32, u32) {
    let (x1, y1) = lon_lat_to_tile(bbox.0, bbox.1, zoom);
    let (x2, y2) = lon_lat_to_tile(bbox.2, bbox.3, zoom);
    // TODO Not sure why y gets swapped sometimes
    (x1, y1.min(y2), x2, y2.max(y1))
}

// Thanks to https://github.com/MilesMcBain/slippymath/blob/master/R/slippymath.R
// Use https://crates.io/crates/tile-grid or something instead?
// Alternatively https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Python
fn lon_lat_to_tile(lon: f64, lat: f64, zoom: u32) -> (u32, u32) {
    let lon_radians = lon.to_radians();
    let lat_radians = lat.to_radians();

    let x = lon_radians;
    let y = lat_radians.tan().asinh();

    let x = (1.0 + (x / f64::consts::PI)) / 2.0;
    let y = (1.0 - (y / f64::consts::PI)) / 2.0;

    let num_tiles = 2u32.pow(zoom) as f64;

    (
        (x * num_tiles).floor() as u32,
        (y * num_tiles).floor() as u32,
    )
}

pub fn tile_to_bbox(tile_x: u32, tile_y: u32, zoom: u32) -> (f64, f64, f64, f64) {
    let (x1, y1) = tile_to_lon_lat(tile_x, tile_y, zoom);
    let (x2, y2) = tile_to_lon_lat(tile_x + 1, tile_y + 1, zoom);
    (x1, y1, x2, y2)
}

// From https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames
fn tile_to_lon_lat(tile_x: u32, tile_y: u32, zoom: u32) -> (f64, f64) {
    let n = f64::consts::PI - 2.0 * f64::consts::PI * (tile_y as f64) / (zoom as f64).exp2();
    let lon = (tile_x as f64) / (zoom as f64).exp2() * 360.0 - 180.0;
    let lat = 180.0 / f64::consts::PI * (0.5 * (n.exp() - (-n).exp())).atan();
    (lon, lat)
}
