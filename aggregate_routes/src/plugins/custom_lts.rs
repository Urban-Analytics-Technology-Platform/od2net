use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::Result;

use lts::{Tags, LTS};

pub fn external_command(command: &str, tags_batch: Vec<Tags>) -> Result<Vec<LTS>> {
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
