use std::collections::HashMap;
use std::time::Instant;

use anyhow::Result;
use futures::{stream, StreamExt};
use geojson::{GeoJson, Value};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    // Manually tuned
    let concurrency = 10;

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Give a .geojson with requests as input");
    }

    let requests = get_requests(&args[1])?;
    let num_requests = requests.len();
    println!("Making {num_requests} requests with concurrency = {concurrency}");

    let start = Instant::now();
    let results = stream::iter(requests)
        .map(|req| tokio::spawn(async { req.calculate_route().await }))
        .buffer_unordered(concurrency);

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
                        println!("Request failed: {err}");
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

fn get_requests(path: &str) -> Result<Vec<Request>> {
    let gj = std::fs::read_to_string(path)?.parse::<GeoJson>()?;
    let mut requests = Vec::new();
    if let GeoJson::FeatureCollection(collection) = gj {
        for feature in collection.features {
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
    // Pretend we have more requests
    let repeated: Vec<_> = requests
        .into_iter()
        .flat_map(|item| std::iter::repeat(item).take(10))
        .collect();
    Ok(repeated)
}
