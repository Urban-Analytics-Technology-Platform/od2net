#[macro_use]
extern crate log;

use std::sync::Once;

use instant::Instant;
use rstar::RTree;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use od2net::config::InputConfig;
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
    max_requests: usize,
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
    pub fn recalculate(&self, input: JsValue) -> Result<String, JsValue> {
        let input: Input = serde_wasm_bindgen::from_value(input)?;

        // TODO All of this should be configurable
        let requests = self.make_requests(input.lng, input.lat, input.max_requests);
        let num_requests = requests.len();
        info!("Made up {num_requests} requests");
        // TODO Everything here is placeholder
        let config = InputConfig {
            requests: od2net::config::Requests {
                description: "placeholder".to_string(),
                pattern: od2net::config::ODPattern::FromEveryOriginToOneDestination,
                origins_path: "".to_string(),
                destinations_path: "".to_string(),
            },
            cost: od2net::config::CostFunction::Distance,
            uptake: od2net::config::Uptake::Identity,
            lts: od2net::config::LtsMapping::BikeOttawa,
        };

        // Calculate single-threaded, until we figure out web workers
        let mut path_calc = fast_paths::create_calculator(&self.prepared_ch.ch);
        let mut counts = Counts::new();
        let routing_start = Instant::now();
        for request in requests {
            od2net::router::handle_request(
                request,
                &mut counts,
                &mut path_calc,
                &self.closest_intersection,
                &self.prepared_ch,
                &config.uptake,
                &self.network,
            );
        }
        let routing_time = Instant::now().duration_since(routing_start);

        info!("Got counts for {} edges", counts.count_per_edge.len());
        let output_metadata =
            od2net::OutputMetadata::new(config, &counts, num_requests, routing_time);
        let mut gj_bytes = Vec::new();
        self.network
            .write_geojson(
                geojson::FeatureWriter::from_writer(std::io::BufWriter::new(&mut gj_bytes)),
                counts,
                true,
                true,
                &output_metadata,
            )
            .map_err(err_to_js)?;
        let gj_string = String::from_utf8(gj_bytes).map_err(err_to_js)?;

        Ok(gj_string)
    }

    // TODO Start simple. From every node to one destination
    fn make_requests(&self, x2: f64, y2: f64, max_requests: usize) -> Vec<Request> {
        let mut requests = Vec::new();
        for i in self.network.intersections.values() {
            let (x1, y1) = i.to_degrees();
            requests.push(Request { x1, y1, x2, y2 });
            if requests.len() == max_requests {
                break;
            }
        }
        requests
    }
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
