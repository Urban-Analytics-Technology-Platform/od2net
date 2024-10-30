use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::Result;

use crate::config::LtsMapping;
use lts::{Tags, LTS};

pub fn calculate_lts_batch(lts: &LtsMapping, tags_batch: Vec<&Tags>) -> Vec<LTS> {
    match lts {
        LtsMapping::SpeedLimitOnly => tags_batch
            .into_iter()
            .map(|tags| lts::speed_limit_only(tags).0)
            .collect(),
        LtsMapping::BikeOttawa => tags_batch
            .into_iter()
            .map(|tags| lts::bike_ottawa(tags).0)
            .collect(),
        LtsMapping::Walking => tags_batch
            .into_iter()
            .map(|tags| lts::walking(tags).0)
            .collect(),
        LtsMapping::ExternalCommand(command) => external_command(command, tags_batch).unwrap(),
    }
}

fn external_command(command: &str, tags_batch: Vec<&Tags>) -> Result<Vec<LTS>> {
    let args: Vec<&str> = command.split(" ").collect();

    let mut cmd = Command::new(args[0])
        .args(args.into_iter().skip(1))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = cmd.stdin.take() {
        let array_tags: Vec<&HashMap<String, String>> =
            tags_batch.iter().map(|t| t.inner()).collect();
        write!(stdin, "{}", serde_json::to_string(&array_tags)?)?;
    }
    // TODO Intermediate string needed?
    let output = String::from_utf8(cmd.wait_with_output()?.stdout)?;
    let lts_batch: Vec<LTS> = serde_json::from_str(&output)?;
    Ok(lts_batch)
}
