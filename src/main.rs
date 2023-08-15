#[macro_use]
extern crate anyhow;

mod custom_routing;
mod input;
mod node_map;
mod od;
mod osm2network;
mod osrm;
mod plugins;
mod requests;
mod tags;

use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use indicatif::HumanCount;

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config_json = fs_err::read_to_string(&args.config_path)?;
    let config: input::InputConfig = match serde_json::from_str(&config_json) {
        Ok(config) => config,
        Err(err) => panic!("{} is invalid: {err}", args.config_path),
    };
    println!(
        "Using config from {}:\n{}\n",
        args.config_path,
        serde_json::to_string_pretty(&config)?
    );

    // Assume the config file is in the directory for the area
    let directory = Path::new(&args.config_path).parent().unwrap().display();

    let mut start = Instant::now();
    let network = {
        let bin_path = format!("{directory}/network.bin");
        let osm_pbf_path = format!("{directory}/input.osm.pbf");
        println!("Trying to load network from {bin_path}");
        match osm2network::Network::load_from_bin(&bin_path) {
            Ok(network) => network,
            Err(err) => {
                println!("That failed ({err}), so generating it from {osm_pbf_path}");
                osm2network::Network::make_from_pbf(&osm_pbf_path, &bin_path)?
            }
        }
    };
    println!("That took {:?}\n", Instant::now().duration_since(start));

    println!("Loading or generating requests");
    start = Instant::now();
    let requests = match config.requests {
        input::Requests::Odjitter {
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
        input::Requests::Generate {
            pattern,
            origins_path,
            destinations_path,
        } => od::generate(
            pattern,
            &origins_path.unwrap_or_else(|| format!("{directory}/origins.geojson")),
            &destinations_path.unwrap_or_else(|| format!("{directory}/destinations.geojson")),
            args.rng_seed,
        )?,
    };
    println!("That took {:?}\n", Instant::now().duration_since(start));

    start = Instant::now();
    let counts = match config.routing {
        input::Routing::OSRM { concurrency } => {
            osrm::run(&network, requests, concurrency.unwrap_or(10)).await?
        }
        input::Routing::FastPaths { cost } => custom_routing::run(
            &format!("{directory}/ch.bin"),
            &network,
            requests,
            cost,
            &config.uptake,
        )?,
    };

    println!(
        "Got counts for {} edges. That took {:?}",
        HumanCount(counts.count_per_edge.len() as u64),
        Instant::now().duration_since(start)
    );
    println!("There were {} errors\n", HumanCount(counts.errors));

    if !args.no_output_csv {
        println!("Writing output CSV");
        start = Instant::now();
        network.write_csv(&format!("{directory}/output.csv"), &counts)?;
        println!("That took {:?}", Instant::now().duration_since(start));
    }

    println!("Writing output GJ");
    start = Instant::now();
    network.write_geojson(&format!("{directory}/output.geojson"), counts)?;
    println!("That took {:?}", Instant::now().duration_since(start));

    Ok(())
}
