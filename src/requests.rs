use anyhow::Result;
use fs_err::File;
use geojson::{FeatureReader, Geometry, Value};

pub struct Request {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl Request {
    pub fn as_geojson_string(&self) -> String {
        let geometry = Geometry::new(Value::LineString(vec![
            vec![self.x1, self.y1],
            vec![self.x2, self.y2],
        ]));
        serde_json::to_string(&geometry).unwrap()
    }
}

impl Request {
    pub fn load_from_geojson(
        path: &str,
        sample_requests: usize,
        cap_requests: Option<usize>,
    ) -> Result<Vec<Request>> {
        let reader = FeatureReader::from_reader(std::io::BufReader::new(File::open(path)?));
        let mut requests = Vec::new();
        let mut total = 0;
        for feature in reader.features() {
            let feature = feature?;
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
        Ok(requests)
    }
}
