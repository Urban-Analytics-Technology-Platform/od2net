use std::{collections::HashMap, io::Cursor};

use anyhow::Result;
use elevation::GeoTiffElevation;
use geo::prelude::HaversineLength;
use geo::{LineString, Polygon};
use indicatif::HumanCount;
use osm_reader::{Element, NodeID, WayID};
use rstar::primitives::{GeomWithData, Line};
use rstar::RTree;

use super::amenities::is_amenity;
use super::greenspace;
use super::{Edge, Network, Position};
use crate::config::{CostFunction, LtsMapping};
use crate::timer::Timer;
use crate::{plugins, utils};
use lts::{Tags, LTS};

impl Network {
    pub fn make_from_osm(
        input_bytes: &[u8],
        lts: &LtsMapping,
        cost: &mut CostFunction,
        timer: &mut Timer,
        geotiff_bytes: Option<Vec<u8>>,
    ) -> Result<Network> {
        timer.start("Make Network from xml or pbf");
        timer.start("Scrape OSM data");
        let (nodes, ways, amenity_positions, greenspace_polygons) = scrape_elements(input_bytes)?;
        timer.stop();
        println!(
            "  Got {} nodes, {} ways, and {} amenities",
            HumanCount(nodes.len() as u64),
            HumanCount(ways.len() as u64),
            HumanCount(amenity_positions.len() as u64)
        );

        if false {
            let mut writer = geojson::FeatureWriter::from_writer(std::io::BufWriter::new(
                fs_err::File::create("debug_greenspace.geojson")?,
            ));
            for polygon in &greenspace_polygons {
                writer.write_feature(&geojson::Feature::from(geojson::Geometry::from(polygon)))?;
            }
        }

        timer.start("Split into edges");
        let mut network = split_edges(nodes, ways);
        timer.stop();
        println!(
            "  Split into {} edges",
            HumanCount(network.edges.len() as u64),
        );

        // TODO Might be more useful to double-count and just see how many things are within a 50m
        // buffer
        let closest_edge = build_closest_edge(&network, timer);
        timer.start("Match amenities to closest edge");
        let progress = utils::progress_bar_for_count(amenity_positions.len());
        for amenity in amenity_positions {
            progress.inc(1);
            if let Some(edge) = closest_edge.nearest_neighbor(&amenity.to_degrees_array()) {
                network.edges.get_mut(&edge.data).unwrap().nearby_amenities += 1;
            }
        }
        timer.stop();

        timer.start("Calculate LTS for all edges");
        let progress = utils::progress_bar_for_count(network.edges.len());
        // LTS calculations can have high overhead in one case, so calculate them in batches
        let all_keys: Vec<(NodeID, NodeID)> = network.edges.keys().cloned().collect();
        for key_batch in all_keys.chunks(1000) {
            let tags_batch: Vec<&Tags> =
                key_batch.iter().map(|e| &network.edges[&e].tags).collect();
            let lts_batch = plugins::lts::calculate_lts_batch(lts, tags_batch);
            for (key, lts) in key_batch.into_iter().zip(lts_batch) {
                progress.inc(1);
                network.edges.get_mut(&key).unwrap().lts = lts;
            }
        }
        timer.stop();

        if let Some(bytes) = geotiff_bytes {
            timer.start("Calculate elevation for all edges");
            let mut geotiff = GeoTiffElevation::new(Cursor::new(bytes));
            let progress = utils::progress_bar_for_count(network.edges.len());
            for (_, edge) in &mut network.edges {
                progress.inc(1);
                edge.apply_elevation(&mut geotiff);
            }
            timer.stop();
        }

        timer.start("Calculate cost for all edges");
        network.recalculate_cost(cost)?;
        timer.stop();

        timer.stop();
        Ok(network)
    }

    pub fn recalculate_cost(&mut self, cost: &mut CostFunction) -> Result<()> {
        cost.normalize()?;

        let progress = utils::progress_bar_for_count(self.edges.len());
        let all_keys: Vec<(NodeID, NodeID)> = self.edges.keys().cloned().collect();
        for key_batch in all_keys.chunks(1000) {
            let input_batch: Vec<&Edge> = key_batch.iter().map(|e| &self.edges[&e]).collect();
            let output_batch = plugins::cost::calculate_batch(cost, input_batch);
            for (key, cost) in key_batch.into_iter().zip(output_batch) {
                progress.inc(1);

                if let Some((forward_cost, backward_cost)) = cost {
                    let edge = self.edges.get_mut(&key).unwrap();
                    edge.forward_cost = Some(forward_cost);
                    edge.backward_cost = Some(backward_cost);
                };
            }
        }

        Ok(())
    }
}

struct Way {
    tags: Tags,
    nodes: Vec<NodeID>,
}

fn scrape_elements(
    input_bytes: &[u8],
) -> Result<(
    HashMap<NodeID, Position>,
    HashMap<WayID, Way>,
    Vec<Position>,
    Vec<Polygon>,
)> {
    // Scrape every node ID -> position
    let mut nodes = HashMap::new();
    // Scrape every routable road. Just tags and node lists to start.
    let mut ways = HashMap::new();
    let mut amenity_positions = Vec::new();
    let mut greenspace_polygons = Vec::new();

    osm_reader::parse(input_bytes, |elem| match elem {
        Element::Node { id, lon, lat, tags } => {
            let pos = Position::from_degrees(lon, lat);
            nodes.insert(id, pos);

            let tags = Tags::from(tags);
            if is_amenity(&tags) {
                amenity_positions.push(pos);
            }
        }
        Element::Way { id, node_ids, tags } => {
            let tags = Tags::from(tags);
            if is_amenity(&tags) {
                // TODO Calculate a centroid instead
                amenity_positions.push(nodes[&node_ids[0]]);
            }

            if let Some(polygon) = greenspace::get_polygon(&tags, &nodes, &node_ids) {
                greenspace_polygons.push(polygon);
            }

            // If a way crosses the boundary and all nodes aren't present, filter it out up-front
            let all_nodes = node_ids.iter().all(|n| nodes.contains_key(n));

            // Include everything here, and let LTS::NotAllowed later filter some out
            if tags.has("highway") && all_nodes {
                ways.insert(
                    id,
                    Way {
                        tags,
                        nodes: node_ids,
                    },
                );
            }
        }
        Element::Relation { .. } => {
            // TODO Handle for amenities. What about when they're large, or might be
            // double-tagged?
            // https://www.openstreetmap.org/relation/14875126
        }
    })?;

    Ok((nodes, ways, amenity_positions, greenspace_polygons))
}

fn split_edges(nodes: HashMap<NodeID, Position>, ways: HashMap<WayID, Way>) -> Network {
    // Count how many ways reference each node
    let mut node_counter: HashMap<NodeID, usize> = HashMap::new();
    for way in ways.values() {
        for node in &way.nodes {
            *node_counter.entry(*node).or_insert(0) += 1;
        }
    }

    // Split each way into edges
    let mut intersections = HashMap::new();
    let mut edges = HashMap::new();
    for (way_id, way) in ways {
        let mut node1 = way.nodes[0];
        let mut pts = Vec::new();

        let num_nodes = way.nodes.len();
        for (idx, node) in way.nodes.into_iter().enumerate() {
            pts.push(nodes[&node]);
            // Edges start/end at intersections between two ways. The endpoints of the way also
            // count as intersections.
            let is_endpoint =
                idx == 0 || idx == num_nodes - 1 || *node_counter.get(&node).unwrap() > 1;
            if is_endpoint && pts.len() > 1 {
                intersections.insert(node1, pts[0]);
                intersections.insert(node, *pts.last().unwrap());
                let length_meters = calculate_length_meters(&pts);
                edges.insert(
                    (node1, node),
                    Edge {
                        way_id,
                        tags: way.tags.clone(),
                        geometry: std::mem::take(&mut pts),
                        length_meters,
                        // Temporary
                        forward_cost: None,
                        backward_cost: None,
                        slope: None,
                        slope_factor: None,
                        lts: LTS::NotAllowed,
                        nearby_amenities: 0,
                    },
                );

                // Start the next edge
                node1 = node;
                pts.push(nodes[&node]);
            }
        }
    }

    Network {
        edges,
        intersections,
    }
}

fn calculate_length_meters(pts: &[Position]) -> f64 {
    let line_string =
        LineString::<f64>::from(pts.iter().map(|pt| pt.to_degrees()).collect::<Vec<_>>());
    line_string.haversine_length()
}

// Split every Edge into individual line segments, and identify by the OSM node ID pair.
// TODO WGS84 caveat, and no linestring primitive?
type EdgeLocation = GeomWithData<Line<[f64; 2]>, (NodeID, NodeID)>;

fn build_closest_edge(network: &Network, timer: &mut Timer) -> RTree<EdgeLocation> {
    timer.start("Building RTree for matching amenities to edges");
    let mut lines = Vec::new();
    for (id, edge) in &network.edges {
        for pair in edge.geometry.windows(2) {
            lines.push(EdgeLocation::new(
                Line::new(pair[0].to_degrees_array(), pair[1].to_degrees_array()),
                *id,
            ));
        }
    }
    let rtree = RTree::bulk_load(lines);
    timer.stop();
    rtree
}
