use std::io::{BufReader, Cursor};

use anyhow::Result;
use fs_err::File;
use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::map_coords::MapCoordsInPlace;
use geo_types::Geometry;
use geojson::FeatureReader;
use mvt::{GeomEncoder, GeomType, MapGrid, Tile, TileId};
use pmtiles2::{util::tile_id, Compression, PMTiles, TileType};
use pointy::Transform;
use rstar::{primitives::CachedEnvelope, RTree, RTreeObject, AABB};

mod math;

struct TreeFeature {
    pub geometry: geo_types::Geometry<f64>,
    pub properties: Option<geojson::JsonObject>,
}

impl From<geojson::Feature> for TreeFeature {
    fn from(feature: geojson::Feature) -> Self {
        let properties = feature.properties;
        // Geometry must exist
        let mut geometry: geo_types::Geometry<f64> = feature.geometry.unwrap().try_into().unwrap();
        geometry.map_coords_in_place(|p| math::wgs84_to_web_mercator([p.x, p.y]).into());
        return Self {
            properties,
            geometry,
        };
    }
}

impl RTreeObject for TreeFeature {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let bbox = self.geometry.bounding_rect().unwrap();
        AABB::from_corners([bbox.min().x, bbox.min().y], [bbox.max().x, bbox.max().y])
    }
}

impl TreeFeature {
    fn get_sort_key(&self, key: &str) -> Option<usize> {
        let props = self.properties.as_ref()?;
        let value = props.get(key)?;
        let num = value.as_f64()?;
        Some(num.round() as usize)
    }
}

fn load_features(
    reader: FeatureReader<BufReader<File>>,
    options: &Options,
) -> Result<(RTree<CachedEnvelope<TreeFeature>>, usize)> {
    let tree_features: Vec<CachedEnvelope<TreeFeature>> = reader
        .features()
        .map(|f| CachedEnvelope::new(f.unwrap().into()))
        .collect();
    let num_features = tree_features.len();
    let tree = RTree::bulk_load(tree_features);
    Ok((tree, num_features))
}

struct Options {
    // Descending
    sort_by_key: Option<String>,
    zoom_levels: Vec<u32>,
    limit_size_bytes: Option<usize>,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Pass in a .geojson file");
    }

    let options = Options {
        sort_by_key: Some("count".to_string()),
        zoom_levels: (0..13).collect(),
        // This is so much less than 500KB, but the final tile size is still big
        limit_size_bytes: Some(200 * 1024),
    };

    let reader = FeatureReader::from_reader(BufReader::new(File::open(&args[1])?));
    let pmtiles = geojson_to_pmtiles(reader, options)?;
    let mut file = File::create("out.pmtiles")?;
    pmtiles.to_writer(&mut file)?;
    Ok(())
}

type PMTilesFile = PMTiles<Cursor<&'static [u8]>>;

fn geojson_to_pmtiles(
    reader: FeatureReader<BufReader<File>>,
    options: Options,
) -> Result<PMTilesFile> {
    let (r_tree, feature_count) = load_features(reader, &options)?;

    let root_envelope = r_tree.root().envelope();

    let bbox = math::BBox::new(
        root_envelope.lower()[0],
        root_envelope.lower()[1],
        root_envelope.upper()[0],
        root_envelope.upper()[1],
    );

    println!("bbox of {} features: {:?}", feature_count, bbox);

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
    pmtiles.min_zoom = options.zoom_levels[0] as u8;
    pmtiles.max_zoom = *options.zoom_levels.last().unwrap() as u8;
    pmtiles.center_zoom = 7;
    pmtiles.center_longitude = -1.1425781;
    pmtiles.center_latitude = 53.904306;

    let map_grid = MapGrid::default();

    for z in &options.zoom_levels {
        let z = *z;
        for x in 0..2u32.pow(z) {
            for y in 0..2u32.pow(z) {
                let tile_id = TileId::new(x, y, z)?;
                let tbounds = map_grid.tile_bbox(tile_id);
                let features = r_tree.locate_in_envelope_intersecting(&AABB::from_corners(
                    [tbounds.x_min(), tbounds.y_min()],
                    [tbounds.x_max(), tbounds.y_max()],
                ));
                // TODO And figure out clipping
                make_tile(tile_id, &mut pmtiles, features.collect(), &options)?;
            }
        }
    }

    Ok(pmtiles)
}

fn make_tile(
    current_tile_id: TileId,
    pmtiles: &mut PMTilesFile,
    mut features: Vec<&CachedEnvelope<TreeFeature>>,
    options: &Options,
) -> Result<()> {
    // We have to do this to each result from RTree, because order is of course not maintained
    // between internal buckets
    if let Some(ref key) = options.sort_by_key {
        features.sort_by_key(|f| f.get_sort_key(key).unwrap_or(0));
        features.reverse();
    }

    let web_mercator_transform = MapGrid::default();
    let transform = web_mercator_transform.tile_transform(current_tile_id);
    let mut tile = Tile::new(4096);

    let mut layer = tile.create_layer("layer1");

    let mut bytes_so_far = 0;
    let mut skipped = false;
    for feature in features {
        let mut b = GeomEncoder::new(GeomType::Linestring, Transform::default());

        let mut any = false;
        if let Geometry::LineString(ref line_string) = feature.geometry {
            for pt in line_string {
                // Transform to mercator
                // let mercator_pt = math::wgs84_to_web_mercator([pt[0], pt[1]]);
                // Transform to 0-1 tile coords (not sure why this doesnt work with passing the
                // transform through)
                let transformed_pt = transform * (pt.x, pt.y);

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

        if !any {
            // This wasn't a LineString. Totally skip.
            // TODO Fix upstream, because b.encode() didn't fail and wound up generating something
            // that breaks the protobuf parsing in the frontend
            continue;
        }

        let encoded = b.encode()?;
        bytes_so_far += encoded.len();
        // TODO Note we don't use the layer size, because it's expensive to constantly protobuf
        // encode it. This could overcount (ignoring properties) but also undercount (the encoded
        // geometry is further compacted by protobuf?)
        if let Some(limit) = options.limit_size_bytes {
            if bytes_so_far > limit {
                skipped = true;
                break;
            }
        }

        let id = layer.num_features() as u64;
        // The ownership swaps between layer and write_feature due to how feature properties are
        // encoded
        let mut write_feature = layer.into_feature(encoded);
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
        "Added {} features into {}, costing {} bytes{}",
        num_features,
        current_tile_id,
        // TODO Maybe this is slow and we should use to_bytes() once
        tile.compute_size(),
        if skipped {
            " (skipping some features after hitting size limit)"
        } else {
            ""
        }
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
