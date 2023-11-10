use std::collections::HashMap;
use std::iter::Peekable;

use anyhow::Result;
use geo::prelude::HaversineLength;
use geo::{LineString, Polygon};
use indicatif::HumanCount;
use osmpbf::{Element, ElementReader};
use rstar::primitives::{GeomWithData, Line};
use rstar::RTree;
use xmlparser::Token;

use super::amenities::is_amenity;
use super::greenspace;
use super::{Edge, Network, Position};
use crate::config::{CostFunction, LtsMapping};
use crate::timer::Timer;
use crate::{plugins, utils};
use lts::{Tags, LTS};

impl Network {
    pub fn make_from_osm(
        is_pbf: bool,
        // pbf or osm.xml depending on is_pbf
        input_bytes: &[u8],
        lts: &LtsMapping,
        cost: &mut CostFunction,
        timer: &mut Timer,
    ) -> Result<Network> {
        timer.start("Make Network from pbf");
        timer.start("Scrape OSM data");
        let (nodes, ways, amenity_positions, greenspace_polygons) = if is_pbf {
            scrape_elements_from_pbf(input_bytes)?
        } else {
            scrape_elements_from_xml(String::from_utf8(input_bytes.to_vec())?)?
        };
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
        let all_keys: Vec<(i64, i64)> = network.edges.keys().cloned().collect();
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

        timer.start("Calculate cost for all edges");
        network.recalculate_cost(cost)?;
        timer.stop();

        timer.stop();
        Ok(network)
    }

    pub fn recalculate_cost(&mut self, cost: &mut CostFunction) -> Result<()> {
        cost.normalize()?;

        let progress = utils::progress_bar_for_count(self.edges.len());
        let all_keys: Vec<(i64, i64)> = self.edges.keys().cloned().collect();
        for key_batch in all_keys.chunks(1000) {
            let input_batch: Vec<&Edge> = key_batch.iter().map(|e| &self.edges[&e]).collect();
            let output_batch = plugins::cost::calculate_batch(cost, input_batch);
            for (key, cost) in key_batch.into_iter().zip(output_batch) {
                progress.inc(1);
                self.edges.get_mut(&key).unwrap().cost = cost;
            }
        }

        Ok(())
    }
}

struct Way {
    tags: Tags,
    nodes: Vec<i64>,
}

fn scrape_elements_from_pbf(
    input_bytes: &[u8],
) -> Result<(
    HashMap<i64, Position>,
    HashMap<i64, Way>,
    Vec<Position>,
    Vec<Polygon>,
)> {
    // Scrape every node ID -> position
    let mut nodes = HashMap::new();
    // Scrape every routable road. Just tags and node lists to start.
    let mut ways = HashMap::new();
    let mut amenity_positions = Vec::new();
    let mut greenspace_polygons = Vec::new();

    let reader = ElementReader::new(input_bytes);
    // TODO par_map_reduce would be fine if we can merge the hashmaps; there should be no repeated
    // keys
    reader.for_each(|element| {
        match element {
            Element::Node(node) => {
                let pos = Position {
                    lon: node.decimicro_lon(),
                    lat: node.decimicro_lat(),
                };
                // TODO Handle TagIter and DenseTagIter instead of this
                let mut tags = Tags::new();
                for (k, v) in node.tags() {
                    tags.insert(k, v);
                }

                nodes.insert(node.id(), pos);
                if is_amenity(&tags) {
                    amenity_positions.push(pos);
                }
            }
            Element::DenseNode(node) => {
                let pos = Position {
                    lon: node.decimicro_lon(),
                    lat: node.decimicro_lat(),
                };
                let mut tags = Tags::new();
                for (k, v) in node.tags() {
                    tags.insert(k, v);
                }

                nodes.insert(node.id(), pos);
                if is_amenity(&tags) {
                    amenity_positions.push(pos);
                }
            }
            Element::Way(way) => {
                let mut tags = Tags::new();
                for (k, v) in way.tags() {
                    tags.insert(k, v);
                }

                if is_amenity(&tags) {
                    // TODO Calculate a centroid instead
                    let pos = nodes[&way.refs().next().unwrap()];
                    amenity_positions.push(pos);
                }

                if let Some(polygon) = greenspace::get_polygon(&tags, &nodes, &way) {
                    greenspace_polygons.push(polygon);
                }

                // Include everything here, and let LTS::NotAllowed later filter some out
                if tags.has("highway") {
                    ways.insert(
                        way.id(),
                        Way {
                            tags,
                            nodes: way.refs().collect(),
                        },
                    );
                }
            }
            Element::Relation(_) => {
                // TODO Handle for amenities. What about when they're large, or might be
                // double-tagged?
                // https://www.openstreetmap.org/relation/14875126
            }
        }
    })?;

    Ok((nodes, ways, amenity_positions, greenspace_polygons))
}

// Ripped out from osm2streets
// TODO Refactor something osmio-like
fn scrape_elements_from_xml(
    input_string: String,
) -> Result<(
    HashMap<i64, Position>,
    HashMap<i64, Way>,
    Vec<Position>,
    Vec<Polygon>,
)> {
    // Scrape every node ID -> position
    let mut nodes = HashMap::new();
    // Scrape every routable road. Just tags and node lists to start.
    let mut ways = HashMap::new();
    let mut amenity_positions = Vec::new();
    let mut _greenspace_polygons = Vec::new();

    // We use the lower-level xmlparser instead of roxmltree to reduce peak memory usage in
    // large files.
    // TODO Reconsider -- large inputs should be pbf anyway
    let mut reader = XmlElementReader {
        tokenizer: xmlparser::Tokenizer::from(input_string.as_str()),
    }
    .peekable();

    while let Some(obj) = reader.next() {
        match obj.name {
            "node" => {
                let id = obj.attribute("id").parse::<i64>().unwrap();
                let pos = Position::from_degrees(
                    obj.attribute("lon").parse::<f64>().unwrap(),
                    obj.attribute("lat").parse::<f64>().unwrap(),
                );
                let tags = read_tags(&mut reader);

                nodes.insert(id, pos);
                if is_amenity(&tags) {
                    amenity_positions.push(pos);
                }
            }
            "way" => {
                let id = obj.attribute("id").parse::<i64>().unwrap();

                let mut node_refs = Vec::new();
                while reader.peek().map(|x| x.name == "nd").unwrap_or(false) {
                    let node_ref = reader.next().unwrap();
                    node_refs.push(node_ref.attribute("ref").parse::<i64>().unwrap());
                }

                // We assume <nd>'s come before <tag>'s
                let tags = read_tags(&mut reader);

                if is_amenity(&tags) {
                    // TODO Calculate a centroid instead
                    amenity_positions.push(nodes[&node_refs[0]]);
                }

                // TODO
                /*if let Some(polygon) = greenspace::get_polygon(&tags, &nodes, &way) {
                    greenspace_polygons.push(polygon);
                }*/

                // Include everything here, and let LTS::NotAllowed later filter some out
                if tags.has("highway") {
                    ways.insert(
                        id,
                        Way {
                            tags,
                            nodes: node_refs,
                        },
                    );
                }
            }
            "relation" => {
                // TODO
            }
            _ => {}
        }
    }

    Ok((nodes, ways, amenity_positions, _greenspace_polygons))
}

fn read_tags(reader: &mut Peekable<XmlElementReader>) -> Tags {
    let mut tags = Tags::new();

    while reader.peek().map(|x| x.name == "tag").unwrap_or(false) {
        let obj = reader.next().unwrap();
        let key = obj.attribute("k");
        let value = obj.attribute("v");
        tags.insert(key, unescape(value).unwrap());
    }

    tags
}

// Reads one element with attributes at a time. Ignores/flattens nested elements.
struct XmlElementReader<'a> {
    tokenizer: xmlparser::Tokenizer<'a>,
}

struct XmlElement<'a> {
    name: &'a str,
    attributes: HashMap<&'a str, &'a str>,
}

impl<'a> XmlElement<'a> {
    fn attribute(&self, key: &str) -> &str {
        self.attributes.get(key).unwrap()
    }
}

impl<'a> Iterator for XmlElementReader<'a> {
    type Item = XmlElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut name: Option<&'a str> = None;
        let mut attributes = HashMap::new();
        loop {
            match self.tokenizer.next()?.unwrap() {
                Token::ElementStart { local, .. } => {
                    assert!(name.is_none());
                    assert!(attributes.is_empty());
                    name = Some(local.as_str());
                }
                Token::Attribute { local, value, .. } => {
                    assert!(name.is_some());
                    attributes.insert(local.as_str(), value.as_str());
                }
                Token::ElementEnd { .. } => {
                    if name.is_none() {
                        assert!(attributes.is_empty());
                        continue;
                    }

                    return Some(XmlElement {
                        name: name.unwrap(),
                        attributes,
                    });
                }
                _ => {}
            }
        }
    }
}

// Copied from https://github.com/Florob/RustyXML, Apache licensed. Unescapes all valid XML
// entities in a string.
fn unescape(input: &str) -> Result<String> {
    let mut result = String::with_capacity(input.len());

    let mut it = input.split('&');

    // Push everything before the first '&'
    if let Some(sub) = it.next() {
        result.push_str(sub);
    }

    for sub in it {
        match sub.find(';') {
            Some(idx) => {
                let ent = &sub[..idx];
                match ent {
                    "quot" => result.push('"'),
                    "apos" => result.push('\''),
                    "gt" => result.push('>'),
                    "lt" => result.push('<'),
                    "amp" => result.push('&'),
                    ent => {
                        let val = if ent.starts_with("#x") {
                            u32::from_str_radix(&ent[2..], 16).ok()
                        } else if ent.starts_with('#') {
                            u32::from_str_radix(&ent[1..], 10).ok()
                        } else {
                            None
                        };
                        match val.and_then(char::from_u32) {
                            Some(c) => result.push(c),
                            None => bail!("&{};", ent),
                        }
                    }
                }
                result.push_str(&sub[idx + 1..]);
            }
            None => bail!("&".to_owned() + sub),
        }
    }
    Ok(result)
}

fn split_edges(nodes: HashMap<i64, Position>, ways: HashMap<i64, Way>) -> Network {
    // Count how many ways reference each node
    let mut node_counter: HashMap<i64, usize> = HashMap::new();
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
                        cost: None,
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
type EdgeLocation = GeomWithData<Line<[f64; 2]>, (i64, i64)>;

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
