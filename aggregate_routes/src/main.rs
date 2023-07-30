use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use futures::{stream, StreamExt};
use geojson::{GeoJson, Value};
use indicatif::{HumanCount, ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Serialize;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    geojson_input: String,
    #[clap(long, default_value_t = 10)]
    concurrency: usize,
    /// A percent (0 to 1000 -- note NOT 100) of requests to use
    #[clap(long, default_value_t = 1000)]
    sample_requests: usize,
    #[clap(long, default_value = "counts.bin")]
    output_counts: String,
}

#[derive(Serialize)]
struct Counts {
    count_per_edge: HashMap<(i64, i64), usize>,
    errors: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut start = Instant::now();
    println!("Loading requests from {}", args.geojson_input);
    let requests = Request::load_from_geojson(&args.geojson_input, args.sample_requests)?;
    println!("That took {:?}", Instant::now().duration_since(start));

    let num_requests = requests.len();
    println!(
        "Making {} requests with concurrency = {}",
        HumanCount(num_requests as u64),
        args.concurrency
    );

    start = Instant::now();
    let results = stream::iter(requests)
        .map(|req| tokio::spawn(async { req.calculate_route().await }))
        .buffer_unordered(args.concurrency);

    // Count routes per node pairs
    let progress = ProgressBar::new(num_requests as u64).with_style(ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap());
    let mut counts = Counts {
        count_per_edge: HashMap::new(),
        errors: 0,
    };
    results
        .fold(&mut counts, |accumulate, future| async {
            progress.inc(1);
            // TODO Flatten
            match future {
                Ok(result) => match result {
                    Ok(nodes) => {
                        for pair in nodes.windows(2) {
                            *accumulate
                                .count_per_edge
                                .entry((pair[0], pair[1]))
                                .or_insert(0) += 1;
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

    println!(
        "Got counts for {} edges. That took {:?}",
        counts.count_per_edge.len(),
        Instant::now().duration_since(start)
    );
    println!("There were {} errors", HumanCount(counts.errors));

    println!("Writing to {}", args.output_counts);
    let writer = BufWriter::new(File::create(&args.output_counts).unwrap());
    bincode::serialize_into(writer, &counts).unwrap();

    Ok(())
}

struct Request {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl Request {
    // Returns OSM node IDs
    async fn calculate_route(self) -> Result<Vec<i64>> {
        // TODO How to share, and does it matter?
        let client = Client::new();

        // Alternatively, try bindings (https://crates.io/crates/rsc_osrm)
        let body = client
            .get(format!(
                "http://localhost:5000/route/v1/driving/{},{};{},{}",
                self.x1, self.y1, self.x2, self.y2
            ))
            .query(&[
                ("overview", "false"),
                ("alternatives", "false"),
                ("steps", "false"),
                ("annotations", "nodes"),
            ])
            .send()
            .await?
            .text()
            .await?;
        let json_value: serde_json::Value = serde_json::from_str(&body)?;
        let nodes: Vec<i64> = serde_json::from_value(
            json_value["routes"][0]["legs"][0]["annotation"]["nodes"].clone(),
        )?;
        Ok(nodes)
    }

    fn load_from_geojson(path: &str, sample_requests: usize) -> Result<Vec<Request>> {
        let gj = std::fs::read_to_string(path)?.parse::<GeoJson>()?;
        let mut requests = Vec::new();
        let mut total = 0;
        if let GeoJson::FeatureCollection(collection) = gj {
            for feature in collection.features {
                total += 1;
                // TODO Off by 1
                if total % 1000 > sample_requests {
                    continue;
                }

                if let Some(geometry) = feature.geometry {
                    if let Value::LineString(line_string) = geometry.value {
                        assert_eq!(2, line_string.len());
                        requests.push(Request {
                            x1: line_string[0][0],
                            y1: line_string[0][1],
                            x2: line_string[1][0],
                            y2: line_string[1][1],
                        });
                    }
                }
            }
        }
        Ok(requests)
    }
}
