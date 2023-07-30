use std::collections::HashMap;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use futures::{stream, StreamExt};
use geojson::{GeoJson, Value};
use indicatif::{HumanCount, ProgressBar, ProgressStyle};
use reqwest::Client;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    geojson_input: String,
    #[clap(long, default_value_t = 10)]
    concurrency: usize,
    /// A percent (0 to 1000 -- note NOT 100) of requests to use
    #[clap(long, default_value_t = 1000)]
    sample_requests: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Loading requests from {}", args.geojson_input);
    let requests = get_requests(&args.geojson_input, args.sample_requests)?;
    let num_requests = requests.len();
    println!(
        "Making {} requests with concurrency = {}",
        HumanCount(num_requests as u64),
        args.concurrency
    );

    let start = Instant::now();
    let results = stream::iter(requests)
        .map(|req| tokio::spawn(async { req.calculate_route().await }))
        .buffer_unordered(args.concurrency);

    // Count routes per node pairs
    let progress = ProgressBar::new(num_requests as u64).with_style(ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap());
    let mut count_per_edge: HashMap<(i64, i64), usize> = HashMap::new();
    results
        .fold(&mut count_per_edge, |accumulate_count, future| async {
            progress.inc(1);
            // TODO Flatten
            match future {
                Ok(result) => match result {
                    Ok(nodes) => {
                        for pair in nodes.windows(2) {
                            *accumulate_count.entry((pair[0], pair[1])).or_insert(0) += 1;
                        }
                    }
                    Err(err) => {
                        // TODO Usually the API being overloaded
                        if false {
                            println!("Request failed: {err}");
                        }
                        // TODO Figure out a nice way to maintain a counter. Having problems with
                        // folding two things and the progress bar
                    }
                },
                Err(err) => {
                    println!("Tokio error: {err}");
                }
            }
            accumulate_count
        })
        .await;
    progress.finish();

    println!(
        "Got counts for {} edges. That took {:?}",
        count_per_edge.len(),
        Instant::now().duration_since(start)
    );

    Ok(())
}

#[derive(Clone)]
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
}

fn get_requests(path: &str, sample_requests: usize) -> Result<Vec<Request>> {
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
