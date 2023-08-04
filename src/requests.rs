use anyhow::Result;
use geojson::{GeoJson, Value};
use reqwest::Client;

pub struct Request {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl Request {
    // Returns OSM node IDs, using OSRM
    pub async fn calculate_route(self) -> Result<Vec<i64>> {
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

    pub fn load_from_geojson(
        path: &str,
        sample_requests: usize,
        cap_requests: Option<usize>,
    ) -> Result<Vec<Request>> {
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
                if let Some(cap) = cap_requests {
                    if requests.len() == cap {
                        break;
                    }
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
