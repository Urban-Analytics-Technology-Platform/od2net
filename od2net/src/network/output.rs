use std::io::{BufWriter, Write};

use anyhow::Result;
use fs_err::File;
use geojson::{feature::Id, Feature, FeatureWriter, Geometry, JsonObject, JsonValue, Value};
use indicatif::HumanCount;
use osm_reader::NodeID;

use super::{Counts, Edge, Network};
use crate::OutputMetadata;

impl Edge {
    fn to_geojson(
        &self,
        node1: NodeID,
        node2: NodeID,
        count: f64,
        id: usize,
        output_osm_tags: bool,
    ) -> Feature {
        let mut feature = self.to_base_geojson(id, node1, node2, output_osm_tags);
        feature.set_property("count", count);
        feature
    }

    pub fn to_geojson_for_detailed_output(
        &self,
        node1: NodeID,
        node2: NodeID,
        geometry_forwards: bool,
    ) -> Feature {
        let mut feature = self.to_base_geojson(0, node1, node2, true);
        feature.id = None;
        if !geometry_forwards {
            if let Some(ref mut geometry) = feature.geometry {
                if let Value::LineString(ref mut pts) = geometry.value {
                    pts.reverse();
                }
            }
        }
        feature
    }

    fn to_base_geojson(
        &self,
        id: usize,
        node1: NodeID,
        node2: NodeID,
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
        properties.insert("way".to_string(), JsonValue::from(self.way_id.0));
        properties.insert("node1".to_string(), JsonValue::from(node1.0));
        properties.insert("node2".to_string(), JsonValue::from(node2.0));
        if let (Some(forward_cost), Some(backward_cost)) = (self.forward_cost, self.backward_cost) {
            // Either both costs are defined or none of them are so the expression above should
            // be enough.
            properties.insert("forward_cost".to_string(), serde_json::to_value(forward_cost).unwrap());
            properties.insert("backward_cost".to_string(), serde_json::to_value(backward_cost).unwrap());
            properties.insert(
                "length".to_string(),
                serde_json::to_value(self.length_meters).unwrap(),
            );

            // put the slope in here since if the cost is None there is no need to pull in the
            // slope.
            if let Some(slope) = self.slope {
                properties.insert("slope".to_string(), serde_json::to_value(slope).unwrap());
            };  
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

    /// Output debug info per edge, without any counts
    pub fn to_debug_geojson(&self) -> Result<String> {
        let mut gj_bytes = Vec::new();
        {
            let mut writer = FeatureWriter::from_writer(BufWriter::new(&mut gj_bytes));
            let mut id_counter = 0;
            for ((node1, node2), edge) in &self.edges {
                id_counter += 1;
                writer.write_feature(&edge.to_base_geojson(id_counter, *node1, *node2, true))?;
            }
            writer.finish()?;
        }
        Ok(String::from_utf8(gj_bytes)?)
    }
}
