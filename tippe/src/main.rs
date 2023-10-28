use std::io::{BufReader, Cursor};

use anyhow::Result;
use fs_err::File;
use geojson::{Feature, FeatureReader, Value};
use mvt::{GeomEncoder, GeomType, MapGrid, Tile, TileId};
use pmtiles2::{util::tile_id, Compression, PMTiles, TileType};
use pointy::Transform;

use math::BBox;

mod math;

// TODO Final result is weird and squiggly -- maybe that's fixed now?

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Pass in a .geojson file");
    }

    let zoom_levels: Vec<u32> = (0..15).collect();

    let reader = FeatureReader::from_reader(BufReader::new(File::open(&args[1])?));
    let pmtiles = geojson_to_pmtiles(reader, zoom_levels)?;
    let mut file = File::create("out.pmtiles")?;
    pmtiles.to_writer(&mut file)?;
    Ok(())
}

type PMTilesFile = PMTiles<Cursor<&'static [u8]>>;

fn geojson_to_pmtiles(
    reader: FeatureReader<BufReader<File>>,
    zoom_levels: Vec<u32>,
) -> Result<PMTilesFile> {
    // TODO Put these in an rtree or similar. For now, just read all at once.
    let mut features = Vec::new();
    for f in reader.features() {
        features.push(f?);
    }

    let bbox = BBox::from_geojson(&features);
    println!("bbox of {} features: {:?}", features.len(), bbox);

    let mut pmtiles = PMTiles::new(TileType::Mvt, Compression::None);
    pmtiles.meta_data = Some(serde_json::json!(
        {
            "antimeridian_adjusted_bounds":"-180,-90,180,90",
            "vector_layers": [
            {
                "id": "layer1",
                "minzoom": 0,
                "maxzoom": 15,
                "fields": {
                    "key": "String"
                }
            }
            ]
        }
    ));
    // TODO Calculate from bbox and other things
    pmtiles.min_latitude = -90.0;
    pmtiles.min_longitude = -180.0;
    pmtiles.max_latitude = 90.0;
    pmtiles.max_longitude = 180.0;
    pmtiles.min_zoom = zoom_levels[0] as u8;
    pmtiles.max_zoom = *zoom_levels.last().unwrap() as u8;
    pmtiles.center_zoom = 7;
    pmtiles.center_longitude = -1.1425781;
    pmtiles.center_latitude = 53.904306;

    for zoom in zoom_levels {
        let (x_min, y_min, x_max, y_max) = bbox.to_tiles(zoom);
        //println!("for {zoom}:    {x_min} to {x_max},   {y_min} to {y_max}");
        // TODO Inclusive or not?
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                // TODO Filter features that belong in this tile
                // TODO And figure out clipping
                make_tile(TileId::new(x, y, zoom)?, &mut pmtiles, &features)?;
            }
        }
    }

    Ok(pmtiles)
}

fn make_tile(
    current_tile_id: TileId,
    pmtiles: &mut PMTilesFile,
    features: &Vec<Feature>,
) -> Result<()> {
    // TODO We don't even need this! Just do the filtering below
    //let tile_bbox = BBox::from_tile(current_tile_id.x(), current_tile_id.y(), current_tile_id.z());

    let web_mercator_transform = MapGrid::default();
    let transform = web_mercator_transform.tile_transform(current_tile_id);
    let mut tile = Tile::new(4096);

    let mut layer = tile.create_layer("layer1");

    for feature in features {
        let mut b = GeomEncoder::new(GeomType::Linestring, Transform::default());

        let mut any = false;
        if let Some(ref geometry) = feature.geometry {
            if let Value::LineString(ref line_string) = geometry.value {
                for pt in line_string {
                    // Transform to mercator
                    let mercator_pt = math::wgs84_to_web_mercator([pt[0], pt[1]]);
                    // Transform to 0-1 tile coords (not sure why this doesnt work with passing the
                    // transform through)
                    let transformed_pt = transform * (mercator_pt[0], mercator_pt[1]);

                    // If any part of the LineString is within this tile, keep the whole thing. No
                    // clipping yet.
                    if transformed_pt.x >= 0.0
                        && transformed_pt.x <= 1.0
                        && transformed_pt.y >= 0.0
                        && transformed_pt.y <= 1.0
                    {
                        any = true;
                    }

                    //println!("{:?} becomes {:?} and then {:?}", pt, mercator_pt, transformed_pt);
                    // Same as extent
                    b = b.point(transformed_pt.x * 4096.0, transformed_pt.y * 4096.0)?;
                }
            }
        }

        if !any {
            // This wasn't a LineString. Totally skip.
            // TODO Fix upstream, because b.encode() didn't fail and wound up generating something
            // that breaks the protobuf parsing in the frontend
            continue;
        }

        let id = layer.num_features() as u64;
        // The ownership swaps between layer and write_feature due to how feature properties are
        // encoded
        let mut write_feature = layer.into_feature(b.encode()?);
        write_feature.set_id(id);
        // TODO actual things
        write_feature.add_tag_string("key", "value");
        layer = write_feature.into_layer();
    }

    let num_features = layer.num_features();
    if num_features == 0 {
        // Nothing fit in this tile, just skip it!
        return Ok(());
    }

    tile.add_layer(layer)?;
    println!(
        "Added {} features into {}, costing {} bytes",
        num_features,
        current_tile_id,
        // TODO Maybe this is slow and we should use to_bytes() once
        tile.compute_size(),
    );

    pmtiles.add_tile(
        tile_id(
            current_tile_id.z() as u8,
            current_tile_id.x() as u64,
            current_tile_id.y() as u64,
        ),
        tile.to_bytes()?,
    );

    Ok(())
}
