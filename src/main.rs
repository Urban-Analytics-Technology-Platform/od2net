mod custom_routing;
mod input;
mod node_map;
mod od;
mod osm2network;
mod osrm;
mod plugins;
mod requests;
mod tags;

use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use indicatif::HumanCount;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    /// A JSON string representing an InputConfig
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
    let network = {
        let bin_path = format!("{}/network.bin", config.directory);
        let osm_pbf_path = format!("{}/input.osm.pbf", config.directory);
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
            &origins_path.unwrap_or_else(|| format!("{}/origins.geojson", config.directory)),
            &destinations_path
                .unwrap_or_else(|| format!("{}/destinations.geojson", config.directory)),
        )?,
    };
    println!("That took {:?}\n", Instant::now().duration_since(start));

    start = Instant::now();
    let counts = match config.routing {
        input::Routing::OSRM { concurrency } => {
            osrm::run(&network, requests, concurrency.unwrap_or(10)).await?
        }
        input::Routing::FastPaths { cost } => custom_routing::run(
            &format!("{}/ch.bin", config.directory),
            &network,
            requests,
            cost,
            &config.filter,
        )?,
    };

    println!(
        "Got counts for {} edges. That took {:?}",
        HumanCount(counts.count_per_edge.len() as u64),
        Instant::now().duration_since(start)
    );
    println!(
        "{} routes were ignored based on filters\n",
        HumanCount(counts.filtered_out)
    );
    println!("There were {} errors\n", HumanCount(counts.errors));

    println!("Writing output GJ");
    start = Instant::now();
    network.write_geojson(&format!("{}/output.geojson", config.directory), counts)?;
    println!("That took {:?}", Instant::now().duration_since(start));

    Ok(())
}
