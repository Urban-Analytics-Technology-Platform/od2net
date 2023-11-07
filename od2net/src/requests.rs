use anyhow::Result;
use fs_err::File;

use geojson::{FeatureReader, Geometry, Value};

#[derive(Debug)]
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

    pub fn load_from_geojson(path: String) -> Result<Vec<Self>> {
        let reader = FeatureReader::from_reader(std::io::BufReader::new(File::open(path)?));
        let mut requests = Vec::new();
        for feature in reader.features() {
            let feature = feature?;
            if let Some(geometry) = feature.geometry {
                if let Value::LineString(line_string) = geometry.value {
                    if line_string.len() != 2 {
                        bail!("LineString doesn't have exactly 2 points");
                    }
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
