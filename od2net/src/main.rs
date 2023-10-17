#[macro_use]
extern crate anyhow;

mod config;
mod detailed_route_output;
mod network;
mod node_map;
mod od;
mod plugins;
mod requests;
mod router;
mod timer;
mod utils;

use std::process::Command;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use indicatif::HumanCount;
use serde::Serialize;

use lts::LTS;

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
    /// Don't create a PMTiles file from the GeoJSON output. The results won't be viewable in the
    /// web app.
    #[clap(long)]
    no_output_pmtiles: bool,

    /// Create an `output/metadata.json` file summarizing the run.
    #[clap(long)]
    output_metadata: bool,

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
    let pipeline_start = Instant::now();

    timer.start("Load network");
    let network = {
        let bin_path = format!("{directory}/intermediate/network.bin");
        let osm_pbf_path = format!("{directory}/input/input.osm.pbf");
        println!("Trying to load network from {bin_path}");
        // TODO timer around something fallible is annoying
        match network::Network::load_from_bin(&bin_path) {
            Ok(network) => network,
            Err(err) => {
                println!("That failed ({err}), so generating it from {osm_pbf_path}");
                network::Network::make_from_pbf(
                    &osm_pbf_path,
                    &bin_path,
                    &config.lts,
                    &config.cost,
                    &mut timer,
                )?
            }
        }
    };
    timer.stop();

    timer.start("Loading or generating requests");
    let requests = od::generate_requests(
        &config.requests,
        format!("{directory}/input"),
        args.rng_seed,
        &mut timer,
    )?;
    let num_requests = requests.len();
    println!("Got {} requests", HumanCount(num_requests as u64));
    timer.stop();

    if let Some(num_routes) = args.detailed_routes {
        return detailed_route_output::run(
            num_routes,
            &format!("{directory}/intermediate/ch.bin"),
            &network,
            requests,
            &config.uptake,
            format!("{directory}/output/"),
            &mut timer,
        );
    }

    timer.start("Routing");
    let routing_start = Instant::now();
    let counts = router::run(
        &format!("{directory}/intermediate/ch.bin"),
        &network,
        requests,
        &config.uptake,
        &mut timer,
    )?;
    println!(
        "Got counts for {} edges",
        HumanCount(counts.count_per_edge.len() as u64),
    );
    println!(
        "{} succeeded, and {} failed",
        HumanCount(num_requests as u64 - counts.errors),
        HumanCount(counts.errors),
    );
    let routing_time = Instant::now().duration_since(routing_start);
    timer.stop();

    if !args.no_output_csv {
        timer.start("Writing output CSV");
        network.write_csv(&format!("{directory}/output/counts.csv"), &counts)?;
        timer.stop();
    }

    let mut output_metadata = OutputMetadata {
        config,
        num_origins: counts.count_per_origin.len(),
        num_destinations: counts.count_per_destination.len(),
        num_requests,
        num_succeeded_requests: num_requests - (counts.errors as usize),
        num_failed_requests: counts.errors as usize,
        num_edges_with_count: counts.count_per_edge.len(),
        routing_time_seconds: routing_time.as_secs_f32(),
        total_time_seconds: None,
        tippecanoe_time_seconds: None,
        total_meters_not_allowed: counts.total_distance_by_lts[LTS::NotAllowed as u8 as usize],
        total_meters_lts1: counts.total_distance_by_lts[LTS::LTS1 as u8 as usize],
        total_meters_lts2: counts.total_distance_by_lts[LTS::LTS2 as u8 as usize],
        total_meters_lts3: counts.total_distance_by_lts[LTS::LTS3 as u8 as usize],
        total_meters_lts4: counts.total_distance_by_lts[LTS::LTS4 as u8 as usize],
    };
    timer.start("Writing output GJ");
    network.write_geojson(
        &format!("{directory}/output/output.geojson"),
        counts,
        !args.no_output_od_points,
        !args.no_output_osm_tags,
        &output_metadata,
    )?;
    timer.stop();

    if !args.no_output_pmtiles {
        timer.start("Converting to pmtiles for rendering");
        let tippecanoe_start = Instant::now();
        let mut cmd = Command::new("tippecanoe");
        cmd.arg(format!("{directory}/output/output.geojson"))
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
            .arg(serde_json::to_string(&output_metadata)?);
        println!("Running: {cmd:?}");
        if !cmd.status()?.success() {
            bail!("tippecanoe failed");
        }
        output_metadata.tippecanoe_time_seconds = Some(
            Instant::now()
                .duration_since(tippecanoe_start)
                .as_secs_f32(),
        );
        timer.stop();
    }

    output_metadata.total_time_seconds =
        Some(Instant::now().duration_since(pipeline_start).as_secs_f32());
    drop(timer);
    println!("");
    output_metadata.describe();

    if args.output_metadata {
        let mut file = fs_err::File::create("output/metadata.json")?;
        serde_json::to_writer(&mut file, &output_metadata)?;
    }

    Ok(())
}

// TODO Move, maybe an output.rs with big chunks of network too
#[derive(Serialize)]
pub struct OutputMetadata {
    config: config::InputConfig,
    num_origins: usize,
    num_destinations: usize,
    num_requests: usize,
    num_succeeded_requests: usize,
    num_failed_requests: usize,
    num_edges_with_count: usize,
    routing_time_seconds: f32,
    total_meters_not_allowed: f64,
    total_meters_lts1: f64,
    total_meters_lts2: f64,
    total_meters_lts3: f64,
    total_meters_lts4: f64,
    // These two aren't recorded in the GeoJSON or PMTiles output, because we'd have to go back and
    // update the files!
    total_time_seconds: Option<f32>,
    tippecanoe_time_seconds: Option<f32>,
}

impl OutputMetadata {
    fn describe(&self) {
        println!("Input: {}", self.config.requests.description);
        for (label, count) in [
            ("Origins", self.num_origins),
            ("Destinations", self.num_destinations),
            ("Requests", self.num_requests),
            ("Requests (succeeded)", self.num_succeeded_requests),
            ("Requests (failed)", self.num_failed_requests),
            ("Edges with a count", self.num_edges_with_count),
        ] {
            println!("- {label}: {}", HumanCount(count as u64));
        }
        for (label, meters) in [
            // For bugspotting
            ("not allowed roads", self.total_meters_not_allowed),
            ("LTS 1 roads", self.total_meters_lts1),
            ("LTS 2 roads", self.total_meters_lts2),
            ("LTS 3 roads", self.total_meters_lts3),
            ("LTS 4 roads", self.total_meters_lts4),
        ] {
            let km = meters / 1000.0;
            println!("- Total distance on {label}: {km:.1} km");
        }
    }
}
