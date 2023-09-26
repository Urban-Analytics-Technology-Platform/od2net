use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

use lts::{Tags, LTS};

pub fn external_command(command: &str, tags: Tags) -> Result<(LTS, Vec<String>)> {
    let args: Vec<&str> = command.split(" ").collect();

    let mut cmd = Command::new(args[0])
        .args(args.into_iter().skip(1))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = cmd.stdin.take() {
        write!(stdin, "{}", serde_json::to_string(tags.inner())?)?;
    }
    let output = String::from_utf8(cmd.wait_with_output()?.stdout)?;
    let num = output.trim().parse::<usize>()?;
    let lts = LTS::from_json(num).context("Unknown LTS number")?;
    Ok((lts, Vec::new()))
}
