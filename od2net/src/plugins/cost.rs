use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::Result;
use serde::Serialize;

use crate::config::{CostFunction, GeneralizedCostFunction};
use crate::network::Edge;
use lts::LTS;

pub fn calculate_batch(cost: &CostFunction, input_batch: Vec<&Edge>) -> Vec<Option<usize>> {
    match cost {
        CostFunction::Distance => input_batch.into_iter().map(distance).collect(),
        CostFunction::OsmHighwayType(ref weights) => input_batch
            .into_iter()
            .map(|e| osm_highway_type(e, weights))
            .collect(),
        CostFunction::ByLTS {
            lts1,
            lts2,
            lts3,
            lts4,
        } => input_batch
            .into_iter()
            .map(|e| by_lts(e, *lts1, *lts2, *lts3, *lts4))
            .collect(),
        CostFunction::Generalized(ref params) => input_batch
            .into_iter()
            .map(|e| generalized(e, params))
            .collect(),
        CostFunction::ExternalCommand(command) => external_command(command, input_batch).unwrap(),
    }
}

fn distance(edge: &Edge) -> Option<usize> {
    by_lts(edge, 1.0, 1.0, 1.0, 1.0)
}

fn osm_highway_type(edge: &Edge, weights: &HashMap<String, f64>) -> Option<usize> {
    let weight = weights.get(edge.tags.get("highway").unwrap())?;
    Some((weight * edge.length_meters).round() as usize)
}

fn by_lts(edge: &Edge, lts1: f64, lts2: f64, lts3: f64, lts4: f64) -> Option<usize> {
    let weight = match edge.lts {
        LTS::NotAllowed => {
            return None;
        }
        LTS::LTS1 => lts1,
        LTS::LTS2 => lts2,
        LTS::LTS3 => lts3,
        LTS::LTS4 => lts4,
    };
    Some((weight * edge.length_meters).round() as usize)
}

fn generalized(edge: &Edge, params: &GeneralizedCostFunction) -> Option<usize> {
    let lts_weight = match edge.lts {
        LTS::NotAllowed => {
            return None;
        }
        LTS::LTS1 => params.lts1,
        LTS::LTS2 => params.lts2,
        LTS::LTS3 => params.lts3,
        LTS::LTS4 => params.lts4,
    };

    let amenities_weight = if edge.nearby_amenities < params.minimum_amenities {
        1.0
    } else {
        0.0
    };

    // TODO For now, every edge gets the bad weight
    let greenspace_weight = 1.0;

    // Use the tradeoffs to get a final penalty
    let penalty = (params.tradeoff_lts * lts_weight)
        + (params.tradeoff_amenities * amenities_weight)
        + (params.tradeoff_greenspace * greenspace_weight);

    Some((penalty * edge.length_meters).round() as usize)
}

fn external_command(command: &str, input_batch: Vec<&Edge>) -> Result<Vec<Option<usize>>> {
    let args: Vec<&str> = command.split(" ").collect();

    let mut cmd = Command::new(args[0])
        .args(args.into_iter().skip(1))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = cmd.stdin.take() {
        let input: Vec<EdgeInput> = input_batch
            .iter()
            .map(|edge| EdgeInput {
                osm_tags: edge.tags.inner(),
                lts: edge.lts,
                nearby_amenities: edge.nearby_amenities,
                length_meters: edge.length_meters,
            })
            .collect();
        write!(stdin, "{}", serde_json::to_string(&input)?)?;
    }
    // TODO Intermediate string needed?
    let output = String::from_utf8(cmd.wait_with_output()?.stdout)?;
    let output_batch: Vec<Option<usize>> = serde_json::from_str(&output)?;
    Ok(output_batch)
}

#[derive(Serialize)]
struct EdgeInput<'a> {
    osm_tags: &'a HashMap<String, String>,
    lts: LTS,
    nearby_amenities: usize,
    length_meters: f64,
}
