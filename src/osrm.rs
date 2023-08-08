use anyhow::Result;
use futures::{stream, StreamExt};
use indicatif::{HumanCount, ProgressBar, ProgressStyle};
use reqwest::Client;

use super::{osm2network, requests};

pub async fn run(
    network: &osm2network::Network,
    requests: Vec<requests::Request>,
    concurrency: usize,
) -> Result<osm2network::Counts> {
    // TODO Take and apply Filter here to routes
    let num_requests = requests.len();
    println!(
        "Making {} requests with concurrency = {}",
        HumanCount(num_requests as u64),
        concurrency
    );

    let results = stream::iter(requests)
        .map(|req| tokio::spawn(async { calculate_route(req).await }))
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

                        // TODO count_per_{o,d}
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

// Returns OSM node IDs, using OSRM
async fn calculate_route(req: requests::Request) -> Result<Vec<i64>> {
    // TODO How to share, and does it matter?
    let client = Client::new();

    // Alternatively, try bindings (https://crates.io/crates/rsc_osrm)
    let body = client
        .get(format!(
            "http://localhost:5000/route/v1/driving/{},{};{},{}",
            req.x1, req.y1, req.x2, req.y2
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
    let nodes: Vec<i64> =
        serde_json::from_value(json_value["routes"][0]["legs"][0]["annotation"]["nodes"].clone())?;
    Ok(nodes)
}
