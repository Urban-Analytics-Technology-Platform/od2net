#[macro_use]
extern crate log;

use std::sync::Once;

use wasm_bindgen::prelude::*;

use od2net::network::Network;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct JsNetwork {
    network: Network,
}

#[wasm_bindgen]
impl JsNetwork {
    /// Call with bincoded bytes of a Network
    #[wasm_bindgen(constructor)]
    pub fn new(input_bytes: &[u8]) -> Result<JsNetwork, JsValue> {
        // Panics shouldn't happen, but if they do, console.log them.
        console_error_panic_hook::set_once();
        START.call_once(|| {
            console_log::init_with_level(log::Level::Info).unwrap();
        });

        info!("Got {} bytes, deserializing", input_bytes.len());

        let network: Network = bincode::deserialize(input_bytes).map_err(err_to_js)?;

        Ok(JsNetwork { network })
    }

    /// Given a GeoJSON LineString, generate a name based on the roads at each endpoint
    #[wasm_bindgen(js_name = tmp)]
    pub fn tmp(&self) -> Result<String, JsValue> {
        Ok(format!("{} edges", self.network.edges.len()))
    }
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
