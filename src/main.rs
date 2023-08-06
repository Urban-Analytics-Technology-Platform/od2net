mod custom_routing;
mod input;
mod node_map;
mod od;
mod osm2network;
mod osrm;
mod requests;
mod tags;

use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use indicatif::HumanCount;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    /// Specify the OSM network to use for counts. Either an osm.pbf file (which'll produce a .bin
    /// file) or a .bin file from a prior run
    #[clap(long)]
    network: String,

    /// A JSON string representing an InputConfig
    #[clap(long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config: input::InputConfig = match serde_json::from_str(&args.config) {
        Ok(config) => config,
        Err(err) => panic!("--config is invalid: {err}"),
    };

    let mut start = Instant::now();
    println!("Loading network from {}", args.network);
    let network = if args.network.ends_with(".osm.pbf") {
        osm2network::Network::make_from_pbf(args.network)?
    } else {
        osm2network::Network::load_from_bin(args.network)?
    };
    println!("That took {:?}\n", Instant::now().duration_since(start));

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
        } => od::generate(pattern, &origins_path, &destinations_path)?,
    };
    println!("That took {:?}\n", Instant::now().duration_since(start));

    start = Instant::now();
    let counts = match config.routing {
        input::Routing::OSRM { concurrency } => {
            osrm::run(&network, requests, concurrency.unwrap_or(10)).await?
        }
        input::Routing::Custom => custom_routing::run(&network, requests)?,
    };

    println!(
        "Got counts for {} edges. That took {:?}",
        HumanCount(counts.count_per_edge.len() as u64),
        Instant::now().duration_since(start)
    );
    println!("There were {} errors\n", HumanCount(counts.errors));

    println!("Writing output GJ");
    start = Instant::now();
    network.write_geojson("output.geojson", counts)?;
    println!("That took {:?}", Instant::now().duration_since(start));

    Ok(())
}
