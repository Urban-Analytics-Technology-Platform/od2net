use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::{bike_ottawa, speed_limit_only, Tags, LTS};

#[derive(Deserialize)]
struct Input {
    // TODO Improve this API
    method: String,
    tags: HashMap<String, String>,
}

#[derive(Serialize)]
struct Output {
    lts: LTS,
    messages: Vec<String>,
}

#[wasm_bindgen()]
pub fn calculate(input: JsValue) -> Result<JsValue, JsValue> {
    let input: Input = serde_wasm_bindgen::from_value(input)?;
    let mut tags = Tags::new();
    for (k, v) in input.tags {
        tags.insert(k, v);
    }

    let (lts, messages) = if input.method == "speed_limit_only" {
        speed_limit_only::speed_limit_only(tags)
    } else if input.method == "bike_ottawa" {
        bike_ottawa::bike_ottawa(tags)
    } else {
        (
            LTS::NotAllowed,
            vec![format!("Unknown method {}", input.method)],
        )
    };
    let result = serde_wasm_bindgen::to_value(&Output {
        lts,
        messages,
    })?;
    Ok(result)
}
