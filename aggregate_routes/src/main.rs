#[macro_use]
extern crate anyhow;

mod config;
mod custom_routing;
mod detailed_route_output;
mod node_map;
mod od;
mod osm2network;
mod plugins;
mod requests;
mod timer;

use std::process::Command;

use anyhow::Result;
use clap::Parser;
use indicatif::HumanCount;
use serde::Serialize;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    /// The path to a JSON file representing an InputConfig
    config_path: String,
    /// Specify a random number seed, used only for some generated request patterns, like BetweenZones.
    #[clap(long, default_value_t = 42)]
    rng_seed: u64,

    /// Don't output a CSV file with each edge's counts.
    #[clap(long)]
    no_output_csv: bool,
    /// Don't output origin and destination points in the GeoJSON output, to reduce file size.
    #[clap(long)]
    no_output_od_points: bool,
    /// Don't output OSM tags in the GeoJSON output, to reduce file size.
    #[clap(long)]
    no_output_osm_tags: bool,

    /// Instead of doing what this tool normally does, instead calculate this many routes and write
    /// a separate GeoJSON file for each of them, with full segment-level detail. This will be slow
    /// and take lots of disk if you specify a large number.
    #[clap(long)]
    detailed_routes: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config_json = fs_err::read_to_string(&args.config_path)?;
    let config: config::InputConfig = match serde_json::from_str(&config_json) {
        Ok(config) => config,
        Err(err) => panic!("{} is invalid: {err}", args.config_path),
    };
    println!(
        "Using config from {}:\n{}\n",
        args.config_path,
        serde_json::to_string_pretty(&config)?
    );

    // Assume the config file is in the directory for the area
    let absolute_path = std::fs::canonicalize(&args.config_path).unwrap();
    let directory = absolute_path.parent().unwrap().display();
    fs_err::create_dir_all(format!("{directory}/intermediate"))?;
    fs_err::create_dir_all(format!("{directory}/output"))?;

    let mut timer = timer::Timer::new();

    timer.start("Load network");
    let network = {
        let bin_path = format!("{directory}/intermediate/network.bin");
        let osm_pbf_path = format!("{directory}/input/input.osm.pbf");
        println!("Trying to load network from {bin_path}");
        // TODO timer around something fallible is annoying
        match osm2network::Network::load_from_bin(&bin_path) {
            Ok(network) => network,
            Err(err) => {
                println!("That failed ({err}), so generating it from {osm_pbf_path}");
                osm2network::Network::make_from_pbf(&osm_pbf_path, &bin_path, &mut timer)?
            }
        }
    };
    timer.stop();

    timer.start("Loading or generating requests");
    let requests = match config.requests.clone() {
        config::Requests::Odjitter {
            path,
            sample_requests,
            cap_requests,
        } => {
            println!("Loading requests from {path}");
            requests::Request::load_from_geojson(
                &path,
                sample_requests.unwrap_or(1000),
                cap_requests,
            )?
        }
        config::Requests::Generate {
            pattern,
            origins_path,
            destinations_path,
        } => od::generate(
            pattern,
            format!("{directory}/input"),
            &origins_path.unwrap_or_else(|| format!("{directory}/input/origins.geojson")),
            &destinations_path.unwrap_or_else(|| format!("{directory}/input/destinations.geojson")),
            args.rng_seed,
            &mut timer,
        )?,
    };
    let num_requests = requests.len();
    println!("Got {} requests", HumanCount(num_requests as u64));
    timer.stop();

    if let Some(num_routes) = args.detailed_routes {
        match config.routing {
            config::Routing::FastPaths { cost } => detailed_route_output::run(
                num_routes,
                &format!("{directory}/intermediate/ch.bin"),
                &network,
                requests,
                cost,
                &config.uptake,
                &config.lts,
                format!("{directory}/output/"),
                &mut timer,
            )?,
        }
        return Ok(());
    }

    timer.start("Routing");
    let counts = match config.routing {
        config::Routing::FastPaths { cost } => custom_routing::run(
            &format!("{directory}/intermediate/ch.bin"),
            &network,
            requests,
            cost,
            &config.uptake,
            &mut timer,
        )?,
    };
    println!(
        "Got counts for {} edges",
        HumanCount(counts.count_per_edge.len() as u64),
    );
    println!(
        "{} succeeded, and {} failed",
        HumanCount(num_requests as u64 - counts.errors),
        HumanCount(counts.errors),
    );
    timer.stop();

    if !args.no_output_csv {
        timer.start("Writing output CSV");
        network.write_csv(&format!("{directory}/output/counts.csv"), &counts)?;
        timer.stop();
    }

    let output_metadata = OutputMetadata {
        config,
        num_origins: counts.count_per_origin.len(),
        num_destinations: counts.count_per_destination.len(),
    };
    timer.start("Writing output GJ");
    network.write_geojson(
        &format!("{directory}/output/output.geojson"),
        counts,
        !args.no_output_od_points,
        !args.no_output_osm_tags,
        &output_metadata.config,
    )?;
    timer.stop();

    timer.start("Converting to pmtiles for rendering");
    let status = Command::new("tippecanoe")
        .arg(format!("{directory}/output/output.geojson"))
        .arg("-o")
        .arg(format!("{directory}/output/rnet.pmtiles"))
        .arg("--force") // Overwrite existing output
        .arg("-l")
        .arg("rnet")
        .arg("-zg") // Guess the zoom
        .arg("--drop-fraction-as-needed") // TODO Drop based on low counts
        .arg("--extend-zooms-if-still-dropping")
        // Plumb through the config as a JSON string in the description
        .arg("--description")
        .arg(serde_json::to_string(&output_metadata)?)
        .status()?;
    if !status.success() {
        bail!("tippecanoe failed");
    }
    timer.stop();

    Ok(())
}

// TODO Move, maybe an output.rs with big chunks of osm2network too
#[derive(Serialize)]
struct OutputMetadata {
    config: config::InputConfig,
    num_origins: usize,
    num_destinations: usize,
}
