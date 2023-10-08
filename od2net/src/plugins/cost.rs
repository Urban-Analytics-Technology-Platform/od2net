use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::Result;
use serde::Serialize;

use crate::config::CostFunction;
use crate::network::Edge;
use lts::LTS;

pub fn calculate_batch(cost: &CostFunction, input_batch: Vec<&Edge>) -> Vec<Option<usize>> {
    match cost {
        CostFunction::Distance => input_batch.into_iter().map(distance).collect(),
        CostFunction::AvoidMainRoads => input_batch.into_iter().map(avoid_main_roads).collect(),
        CostFunction::ExternalCommand(command) => external_command(command, input_batch).unwrap(),
    }
}

fn distance(edge: &Edge) -> Option<usize> {
    // TODO Match the lts.ts definition
    if edge.tags.is("bicycle", "no")
        || edge.tags.is("highway", "motorway")
        || edge.tags.is("highway", "proposed")
    {
        return None;
    }

    Some(edge.length_meters.round() as usize)
}

fn avoid_main_roads(edge: &Edge) -> Option<usize> {
    // TODO Match the lts.ts definition
    if edge.tags.is("bicycle", "no")
        || edge.tags.is("highway", "motorway")
        || edge.tags.is("highway", "proposed")
    {
        return None;
    }

    // TODO Reframe this to just penalize by LTS?
    let penalty = if edge.tags.is("highway", "residential") || edge.tags.is("highway", "cycleway") {
        1.0
    } else {
        5.0
    };

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
