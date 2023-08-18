use std::io::BufWriter;

use anyhow::Result;
use fs_err::File;

use super::config::{CostFunction, LtsMapping, Uptake};
use super::custom_routing::{build_ch, build_closest_intersection, PreparedCH};
use super::osm2network::Network;
use super::requests::Request;

pub fn run(
    num_requests: usize,
    ch_path: &str,
    network: &Network,
    requests: Vec<Request>,
    cost: CostFunction,
    uptake: &Uptake,
    lts: LtsMapping,
    output_directory: String,
) -> Result<()> {
    let prepared_ch = build_ch(ch_path, network, cost)?;
    let closest_intersection = build_closest_intersection(network, &prepared_ch.node_map);

    let mut path_calc = fast_paths::create_calculator(&prepared_ch.ch);

    let mut i = 1;
    for req in requests {
        let start = closest_intersection
            .nearest_neighbor(&[req.x1, req.y1])
            .unwrap()
            .data;
        let end = closest_intersection
            .nearest_neighbor(&[req.x2, req.y2])
            .unwrap()
            .data;
        if start == end {
            println!("Skipping degenerate request {} -- the start and end are both http://openstreetmap.org/node/{}", req.as_geojson_string(), prepared_ch.node_map.translate_id(start));
            continue;
        }

        if let Some(path) = path_calc.calc_path(&prepared_ch.ch, start, end) {
            output_detailed_route(
                format!("{output_directory}/route{i}.geojson"),
                path,
                &prepared_ch,
                network,
                lts,
            )?;
            if i == num_requests {
                break;
            }
            i += 1;
        }
    }

    Ok(())
}

fn output_detailed_route(
    filename: String,
    path: fast_paths::ShortestPath,
    prepared_ch: &PreparedCH,
    network: &Network,
    lts: LtsMapping,
) -> Result<()> {
    // TODO Include uptake and stats about the entire route

    let mut features = Vec::new();

    for pair in path.get_nodes().windows(2) {
        let i1 = prepared_ch.node_map.translate_id(pair[0]);
        let i2 = prepared_ch.node_map.translate_id(pair[1]);
        let (edge, geometry_forwards) = if let Some(edge) = network.edges.get(&(i1, i2)) {
            (edge, true)
        } else {
            (network.edges.get(&(i2, i1)).unwrap(), false)
        };
        features.push(edge.to_geojson_for_detailed_output(i1, i2, lts, geometry_forwards));
    }

    let gj = geojson::FeatureCollection {
        features,
        bbox: None,
        foreign_members: None,
    };
    let mut file = BufWriter::new(File::create(filename)?);
    serde_json::to_writer(&mut file, &gj)?;

    Ok(())
}
