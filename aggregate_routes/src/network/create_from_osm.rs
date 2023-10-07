use std::collections::HashMap;
use std::io::BufWriter;

use anyhow::Result;
use fs_err::File;
use geo::prelude::HaversineLength;
use geo::LineString;
use indicatif::{HumanCount, ProgressBar, ProgressStyle};
use osmpbf::{Element, ElementReader};

use super::{Edge, Network, Position};
use crate::config::LtsMapping;
use crate::plugins;
use crate::timer::Timer;
use lts::{Tags, LTS};

impl Network {
    pub fn make_from_pbf(
        osm_pbf_path: &str,
        bin_path: &str,
        lts: &LtsMapping,
        timer: &mut Timer,
    ) -> Result<Network> {
        timer.start("Make Network from pbf");
        timer.start("Scrape OSM data");
        let (nodes, ways) = scrape_elements(osm_pbf_path)?;
        timer.stop();
        println!(
            "  Got {} nodes and {} ways",
            HumanCount(nodes.len() as u64),
            HumanCount(ways.len() as u64)
        );

        timer.start("Split into edges");
        let mut network = split_edges(nodes, ways);
        timer.stop();
        println!(
            "  Split into {} edges",
            HumanCount(network.edges.len() as u64),
        );

        timer.start("Calculate LTS for all edges");
        // TODO Refactor helper?
        let progress = ProgressBar::new(network.edges.len() as u64).with_style(ProgressStyle::with_template(
                "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap());
        // LTS calculations can have high overhead in one case, so calculate them in batches
        let all_keys: Vec<(i64, i64)> = network.edges.keys().cloned().collect();
        for key_batch in all_keys.chunks(1000) {
            let tags_batch: Vec<Tags> = key_batch
                .iter()
                .map(|e| network.edges[&e].cleaned_tags())
                .collect();
            let lts_batch = calculate_lts_batch(lts, tags_batch);
            for (key, lts) in key_batch.into_iter().zip(lts_batch) {
                progress.inc(1);
                network.edges.get_mut(&key).unwrap().lts = lts;
            }
        }
        timer.stop();

        timer.start(format!("Saving to {bin_path}"));
        let writer = BufWriter::new(File::create(bin_path)?);
        bincode::serialize_into(writer, &network)?;
        timer.stop();

        timer.stop();
        Ok(network)
    }
}

struct Way {
    tags: Vec<(String, String)>,
    nodes: Vec<i64>,
}

fn scrape_elements(path: &str) -> Result<(HashMap<i64, Position>, HashMap<i64, Way>)> {
    // Scrape every node ID -> position
    let mut nodes = HashMap::new();
    // Scrape every routable road. Just tags and node lists to start.
    let mut ways = HashMap::new();

    let reader = ElementReader::from_path(path)?;
    // TODO par_map_reduce would be fine if we can merge the hashmaps; there should be no repeated
    // keys
    reader.for_each(|element| {
        match element {
            Element::Node(node) => {
                nodes.insert(
                    node.id(),
                    Position {
                        lon: node.decimicro_lon(),
                        lat: node.decimicro_lat(),
                    },
                );
            }
            Element::DenseNode(node) => {
                nodes.insert(
                    node.id(),
                    Position {
                        lon: node.decimicro_lon(),
                        lat: node.decimicro_lat(),
                    },
                );
            }
            Element::Way(way) => {
                // TODO Improve filtering
                if way.tags().any(|(key, _)| key == "highway") {
                    ways.insert(
                        way.id(),
                        Way {
                            tags: way
                                .tags()
                                .map(|(k, v)| (k.to_string(), v.to_string()))
                                .collect(),
                            nodes: way.refs().collect(),
                        },
                    );
                }
            }
            Element::Relation(_) => {}
        }
    })?;

    Ok((nodes, ways))
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
                        lts: LTS::NotAllowed,
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

fn calculate_lts_batch(lts: &LtsMapping, tags_batch: Vec<Tags>) -> Vec<LTS> {
    match lts {
        LtsMapping::SpeedLimitOnly => tags_batch
            .into_iter()
            .map(|tags| lts::speed_limit_only(tags).0)
            .collect(),
        LtsMapping::BikeOttawa => tags_batch
            .into_iter()
            .map(|tags| lts::bike_ottawa(tags).0)
            .collect(),
        LtsMapping::ExternalCommand(command) => {
            plugins::custom_lts::external_command(command, tags_batch).unwrap()
        }
    }
}
