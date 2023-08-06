mod custom_routing;
mod node_map;
mod od;
mod osm2network;
mod requests;
mod tags;

use std::time::Instant;

use anyhow::Result;
use clap::{Parser, Subcommand};
use futures::{stream, StreamExt};
use indicatif::{HumanCount, ProgressBar, ProgressStyle};

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    /// Specify the OSM network to use for counts. Either an osm.pbf file (which'll produce a .bin
    /// file) or a .bin file from a prior run
    #[clap(long)]
    network: String,

    #[command(subcommand)]
    requests: Requests,

    #[command(subcommand)]
    routing: Routing,
}

#[derive(Subcommand)]
enum Requests {
    Odjitter {
        /// A GeoJSON file with LineString requests
        path: String,
        /// A percent (0 to 1000 -- note NOT 100) of requests to use
        #[clap(long, default_value_t = 1000)]
        sample_requests: usize,
        /// Cap requests to exactly this many.
        #[clap(long)]
        cap_requests: Option<usize>,
    },
    Generate {
        /// Path to a GeoJSON file with origin points to use
        origins_path: String,
        /// Path to a GeoJSON file with destination points to use
        destinations_path: String,
    },
}

#[derive(Subcommand)]
enum Routing {
    OSRM {
        /// How many requests to OSRM to have in-flight at once
        #[clap(long, default_value_t = 10)]
        concurrency: usize,
    },
    Custom,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut start = Instant::now();
    println!("Loading network from {}", args.network);
    let network = if args.network.ends_with(".osm.pbf") {
        osm2network::Network::make_from_pbf(args.network)?
    } else {
        osm2network::Network::load_from_bin(args.network)?
    };
    println!("That took {:?}\n", Instant::now().duration_since(start));

    start = Instant::now();
    let requests = match args.requests {
        Requests::Odjitter {
            path,
            sample_requests,
            cap_requests,
        } => {
            println!("Loading requests from {path}");
            requests::Request::load_from_geojson(&path, sample_requests, cap_requests)?
        }
        Requests::Generate {
            origins_path,
            destinations_path,
        } => od::generate(&origins_path, &destinations_path)?,
    };
    println!("That took {:?}\n", Instant::now().duration_since(start));

    start = Instant::now();
    let counts = if args.use_custom_routing {
        custom_routing::run(&network, requests)?
    } else {
        run_with_osrm(&network, requests, args.concurrency).await?
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

async fn run_with_osrm(
    network: &osm2network::Network,
    requests: Vec<requests::Request>,
    concurrency: usize,
) -> Result<osm2network::Counts> {
    let num_requests = requests.len();
    println!(
        "Making {} requests with concurrency = {}",
        HumanCount(num_requests as u64),
        concurrency
    );

    let results = stream::iter(requests)
        .map(|req| tokio::spawn(async { req.calculate_route().await }))
        .buffer_unordered(concurrency);

    // Count routes per node pairs
    let progress = ProgressBar::new(num_requests as u64).with_style(ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap());
    let mut counts = osm2network::Counts::new();
    results
        .fold(&mut counts, |accumulate, future| async {
            progress.inc(1);
            // TODO Flatten
            match future {
                Ok(result) => match result {
                    Ok(nodes) => {
                        // OSRM returns all nodes, but we only consider some to be intersections
                        // TODO When the route begins or ends with an intermediate non-intersection
                        // node, we don't handle it well yet
                        let mut i1 = nodes[0];
                        let mut last = nodes[0];
                        for node in nodes.into_iter().skip(1) {
                            if network.intersections.contains_key(&node) {
                                *accumulate.count_per_edge.entry((i1, node)).or_insert(0) += 1;
                                i1 = node;
                            }
                            last = node;
                        }
                        if i1 != last && false {
                            println!("We didn't end on an intersection... {i1} to {last}");
                        }
                    }
                    Err(err) => {
                        // TODO Usually the API being overloaded
                        if false {
                            println!("Request failed: {err}");
                        }
                        accumulate.errors += 1;
                    }
                },
                Err(err) => {
                    println!("Tokio error: {err}");
                }
            }
            accumulate
        })
        .await;
    progress.finish();

    Ok(counts)
}
