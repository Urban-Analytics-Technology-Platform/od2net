use geojson::{Geometry, Value};

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
