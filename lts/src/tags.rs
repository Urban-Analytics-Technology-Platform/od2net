use std::collections::HashMap;

/// Convenience functions around a string->string map
pub struct Tags(HashMap<String, String>);

impl Tags {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<K: Into<String>, V: Into<String>>(&mut self, k: K, v: V) {
        self.0.insert(k.into(), v.into());
    }

    pub fn is(&self, k: &str, v: &str) -> bool {
        self.0.get(k) == Some(&v.to_string())
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.0.get(k)
    }

    pub fn has(&self, k: &str) -> bool {
        self.0.contains_key(k)
    }

    pub fn is_any(&self, k: &str, values: Vec<&str>) -> bool {
        if let Some(v) = self.0.get(k) {
            values.contains(&v.as_ref())
        } else {
            false
        }
    }

    // Returns the key and value that match
    pub fn prefix_is_any(&self, key_prefix: &str, values: Vec<&str>) -> Option<(String, String)> {
        for (k, v) in &self.0 {
            if k.starts_with(key_prefix) && values.contains(&v.as_ref()) {
                return Some((k.to_string(), v.to_string()));
            }
        }
        None
    }

    pub fn inner(&self) -> &HashMap<String, String> {
        &self.0
    }
}
