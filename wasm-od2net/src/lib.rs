#[macro_use]
extern crate log;

use std::sync::Once;

use rstar::RTree;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use od2net::config::Uptake;
use od2net::network::{Counts, Network};
use od2net::requests::Request;
use od2net::router::{IntersectionLocation, PreparedCH};
use od2net::timer::Timer;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct JsNetwork {
    network: Network,
    prepared_ch: PreparedCH,
    // TODO Maybe bundle this in PreparedCH and rethink what we serialize
    closest_intersection: RTree<IntersectionLocation>,
}

#[derive(Deserialize)]
struct Input {
    lng: f64,
    lat: f64,
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

        // TODO Recalculate this sometimes, but also, will have to modify the Network in-place
        let mut timer = Timer::new();
        let prepared_ch = od2net::router::just_build_ch(&network, &mut timer);
        let closest_intersection =
            od2net::router::build_closest_intersection(&network, &prepared_ch.node_map, &mut timer);

        Ok(JsNetwork {
            network,
            prepared_ch,
            closest_intersection,
        })
    }

    #[wasm_bindgen()]
    pub fn recalculate(&self, input: JsValue) -> Result<(), JsValue> {
        let input: Input = serde_wasm_bindgen::from_value(input)?;

        // TODO All of this should be configurable
        let requests = self.make_requests(input.lng, input.lat);
        info!("Made up {} requests", requests.len());
        let uptake = Uptake::Identity;

        // Calculate single-threaded, until we figure out web workers
        let mut path_calc = fast_paths::create_calculator(&self.prepared_ch.ch);
        let mut counts = Counts::new();
        for request in requests {
            od2net::router::handle_request(
                request,
                &mut counts,
                &mut path_calc,
                &self.closest_intersection,
                &self.prepared_ch,
                &uptake,
                &self.network,
            );
        }

        info!("Got counts for {} edges", counts.count_per_edge.len());

        Ok(())
    }

    // TODO Start simple. From every node to one destination
    fn make_requests(&self, x2: f64, y2: f64) -> Vec<Request> {
        let mut requests = Vec::new();
        for i in self.network.intersections.values() {
            let (x1, y1) = i.to_degrees();
            requests.push(Request { x1, y1, x2, y2 });
        }
        requests
    }
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
