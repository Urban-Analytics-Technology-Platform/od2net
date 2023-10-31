use std::io::{BufWriter, Write};

use anyhow::Result;
use fs_err::File;
use geojson::{feature::Id, Feature, FeatureWriter, Geometry, JsonObject, JsonValue, Value};
use indicatif::HumanCount;

use super::{Counts, Edge, Network};
use crate::OutputMetadata;

impl Edge {
    fn to_geojson(
        &self,
        node1: i64,
        node2: i64,
        count: f64,
        id: usize,
        output_osm_tags: bool,
    ) -> Feature {
        let geometry = Geometry::new(Value::LineString(
            self.geometry.iter().map(|pt| pt.to_degrees_vec()).collect(),
        ));
        let mut properties = JsonObject::new();
        if output_osm_tags {
            let mut tags = JsonObject::new();
            for (key, value) in self.tags.inner() {
                tags.insert(key.to_string(), JsonValue::from(value.to_string()));
            }
            properties.insert("osm_tags".to_string(), tags.into());
        }
        properties.insert("node1".to_string(), JsonValue::from(node1));
        properties.insert("node2".to_string(), JsonValue::from(node2));
        properties.insert("way".to_string(), JsonValue::from(self.way_id));
        properties.insert("count".to_string(), JsonValue::from(count));
        if let Some(cost) = self.cost {
            properties.insert("cost".to_string(), serde_json::to_value(cost).unwrap());
        }
        properties.insert("lts".to_string(), serde_json::to_value(self.lts).unwrap());
        properties.insert(
            "nearby_amenities".to_string(),
            serde_json::to_value(self.nearby_amenities).unwrap(),
        );
        Feature {
            bbox: None,
            geometry: Some(geometry),
            id: Some(Id::Number(id.into())),
            properties: Some(properties),
            foreign_members: None,
        }
    }

    pub fn to_geojson_for_detailed_output(
        &self,
        node1: i64,
        node2: i64,
        geometry_forwards: bool,
    ) -> Feature {
        let mut pts = self
            .geometry
            .iter()
            .map(|pt| pt.to_degrees_vec())
            .collect::<Vec<_>>();
        if !geometry_forwards {
            pts.reverse();
        }
        let geometry = Geometry::new(Value::LineString(pts));

        let mut properties = JsonObject::new();
        let mut tags = JsonObject::new();
        for (key, value) in self.tags.inner() {
            tags.insert(key.to_string(), JsonValue::from(value.to_string()));
        }
        properties.insert("osm_tags".to_string(), tags.into());
        properties.insert("node1".to_string(), JsonValue::from(node1));
        properties.insert("node2".to_string(), JsonValue::from(node2));
        properties.insert("way".to_string(), JsonValue::from(self.way_id));
        if let Some(cost) = self.cost {
            properties.insert("cost".to_string(), serde_json::to_value(cost).unwrap());
        }
        properties.insert("lts".to_string(), serde_json::to_value(self.lts).unwrap());
        properties.insert(
            "nearby_amenities".to_string(),
            serde_json::to_value(self.nearby_amenities).unwrap(),
        );
        Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: Some(properties),
            foreign_members: None,
        }
    }
}

impl Network {
    pub fn write_geojson<W: std::io::Write>(
        &self,
        mut writer: FeatureWriter<W>,
        counts: Counts,
        output_od_points: bool,
        output_osm_tags: bool,
        output_metadata: &OutputMetadata,
    ) -> Result<()> {
        // Write one feature at a time to avoid memory problems
        writer.write_foreign_member("metadata", output_metadata)?;

        let mut skipped = 0;
        let mut id_counter = 0;
        for ((node1, node2), count) in counts.count_per_edge {
            // TODO Track forwards and backwards counts separately, and optionally merge later?
            if let Some(edge) = self
                .edges
                .get(&(node1, node2))
                .or_else(|| self.edges.get(&(node2, node1)))
            {
                id_counter += 1;
                let feature = edge.to_geojson(node1, node2, count, id_counter, output_osm_tags);
                writer.write_feature(&feature)?;
            } else {
                // TODO We don't handle routes starting or ending in the middle of an edge yet
                //println!("No edge from https://www.openstreetmap.org/node/{node1} to https://www.openstreetmap.org/node/{node2} or vice versa");
                skipped += 1;
            }
        }
        println!(
            "Skipped {} edges (started/ended mid-edge)",
            HumanCount(skipped)
        );

        if output_od_points {
            // Also write origin/destination points with the number of routes to the same file. It
            // hugely bloats the size, but keeping them together is useful right now.

            for (key, counter) in [
                ("origin_count", counts.count_per_origin),
                ("destination_count", counts.count_per_destination),
            ] {
                for (pt, count) in counter {
                    id_counter += 1;
                    let geometry = Geometry::new(Value::Point(pt.to_degrees_vec()));
                    let mut properties = JsonObject::new();
                    properties.insert(key.to_string(), JsonValue::from(count));
                    writer.write_feature(&Feature {
                        bbox: None,
                        geometry: Some(geometry),
                        id: Some(Id::Number(id_counter.into())),
                        properties: Some(properties),
                        foreign_members: None,
                    })?;
                }
            }
        }

        writer.finish()?;

        Ok(())
    }

    pub fn write_csv(&self, path: &str, counts: &Counts) -> Result<()> {
        let mut file = BufWriter::new(File::create(path)?);
        writeln!(file, "way,node1,node2,count")?;

        let mut skipped = 0;
        for ((node1, node2), count) in &counts.count_per_edge {
            if let Some(edge) = self
                .edges
                .get(&(*node1, *node2))
                .or_else(|| self.edges.get(&(*node2, *node1)))
            {
                let way = edge.way_id;
                writeln!(file, "{way},{node1},{node2},{count}")?;
            } else {
                skipped += 1;
            }
        }

        println!(
            "Skipped {} edges (started/ended mid-edge)",
            HumanCount(skipped)
        );
        Ok(())
    }
}
