use std::io::BufReader;

use anyhow::Result;
use fs_err::File;
use geojson::{FeatureReader, Value};
use mvt::{GeomEncoder, GeomType, MapGrid, Tile, TileId};
use pmtiles2::{util::tile_id, Compression, PMTiles, TileType};

fn main() -> Result<()> {
    let web_mercator_transform = MapGrid::default();

    // Everything
    let current_tile_id = TileId::new(0, 0, 0)?;
    let transform = web_mercator_transform.tile_transform(current_tile_id);
    // TODO whats the extent supposed to be?
    let mut tile = Tile::new(4096);
    let mut layer = tile.create_layer("layer1");

    let reader = FeatureReader::from_reader(BufReader::new(File::open(
        "../examples/york/output/output.geojson",
    )?));
    for feature in reader.features() {
        let feature = feature?;
        // TODO or Transform::default
        let mut b = GeomEncoder::new(GeomType::Linestring, transform);

        if let Some(geometry) = feature.geometry {
            if let Value::LineString(line_string) = geometry.value {
                for pt in line_string {
                    b = b.point(pt[0], pt[1])?;
                }
            }
        }
        let id = layer.num_features() as u64;
        let mut write_feature = layer.into_feature(b.encode()?);
        write_feature.set_id(id);
        // TODO actual things
        write_feature.add_tag_string("key", "value");

        // Very weird pattern, but OK
        layer = write_feature.into_layer();
    }
    println!("Got {} features", layer.num_features());
    tile.add_layer(layer)?;

    let mut pmtiles = PMTiles::new(TileType::Mvt, Compression::None);
    let metadata = serde_json::json!(
        {
            "vector_layers": [
            {
                "id": "layer1",
                "minzoom": 0,
                "maxzoom": 1,
                "fields": {
                    "key": "String"
                }
            }
            ]
        }
    );
    pmtiles.meta_data = Some(metadata);
    pmtiles.add_tile(
        tile_id(
            current_tile_id.z() as u8,
            current_tile_id.x() as u64,
            current_tile_id.y() as u64,
        ),
        tile.to_bytes()?,
    );

    let mut file = File::create("out.pmtiles")?;
    pmtiles.to_writer(&mut file)?;
    Ok(())
}
