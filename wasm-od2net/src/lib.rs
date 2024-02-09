#[macro_use]
extern crate log;

use std::sync::Once;

use instant::Instant;
use rstar::RTree;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use od2net::config::{CostFunction, InputConfig};
use od2net::network::{Counts, Network};
use od2net::requests::Request;
use od2net::router::{IntersectionLocation, PreparedCH};
use od2net::timer::Timer;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct JsNetwork {
    network: Network,

    prepared_ch: Option<PreparedCH>,
    // TODO Maybe bundle this in PreparedCH and rethink what we serialize
    closest_intersection: Option<RTree<IntersectionLocation>>,

    // TODO Network should store this, since it's baked in
    last_cost: CostFunction,
}

#[derive(Deserialize)]
struct Input {
    lng: f64,
    lat: f64,
    max_requests: usize,
    cost: CostFunction,
}

#[wasm_bindgen]
impl JsNetwork {
    /// Call with bytes of an osm.pbf or osm.xml string
    #[wasm_bindgen(constructor)]
    pub fn new(input_bytes: &[u8], dem_input_buffer: Option<Box<[u8]>>) -> Result<JsNetwork, JsValue> {
        // Panics shouldn't happen, but if they do, console.log them.
        console_error_panic_hook::set_once();
        
        START.call_once(|| {
            console_log::init_with_level(log::Level::Info).unwrap();
        });
       
        info!("Got {} bytes, parsing as an osm.pbf", input_bytes.len());
        if dem_input_buffer.is_some() {
            let bytes = dem_input_buffer.as_ref().unwrap();
            info!("Dem file detected got {} bytes", bytes.len());
        } else {
            info!("No dem file detected!");
        };
        let mut timer = Timer::new();
        // TODO Default config
        let network = Network::make_from_osm(
            input_bytes,
            &od2net::config::LtsMapping::BikeOttawa,
            &mut CostFunction::Distance,
            &mut timer,
            dem_input_buffer,
        )
        .map_err(err_to_js)?;

        Ok(JsNetwork {
            network,
            prepared_ch: None,
            closest_intersection: None,

            // TODO Part of Network?
            last_cost: CostFunction::Distance,
        })
    }

    /// Takes Input, returns GeoJSON
    #[wasm_bindgen()]
    pub fn recalculate(&mut self, input: JsValue) -> Result<String, JsValue> {
        let input: Input = serde_wasm_bindgen::from_value(input)?;

        if input.cost != self.last_cost || self.prepared_ch.is_none() {
            self.last_cost = input.cost;
            let mut timer = Timer::new();
            info!("Recalculating cost");
            self.network
                .recalculate_cost(&mut self.last_cost)
                .map_err(err_to_js)?;
            self.prepared_ch = Some(od2net::router::just_build_ch(&self.network, &mut timer));
            self.closest_intersection = Some(od2net::router::build_closest_intersection(
                &self.network,
                &self.prepared_ch.as_ref().unwrap().node_map,
                &mut timer,
            ));
        }

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
            cost: self.last_cost.clone(),
            dem: "".to_string(),
            uptake: od2net::config::Uptake::Identity,
            lts: od2net::config::LtsMapping::BikeOttawa,
        };

        // Calculate single-threaded, until we figure out web workers
        let mut path_calc = fast_paths::create_calculator(&self.prepared_ch.as_ref().unwrap().ch);
        let mut counts = Counts::new();
        let routing_start = Instant::now();
        for request in requests {
            od2net::router::handle_request(
                request,
                &mut counts,
                &mut path_calc,
                self.closest_intersection.as_ref().unwrap(),
                self.prepared_ch.as_ref().unwrap(),
                &config.uptake,
                &self.network,
            );
        }
        let routing_time = Instant::now().duration_since(routing_start);

        if counts.count_per_edge.len() == 0 {
            // All requests failed?
            warn!("All requests failed, empty result");
            return Ok("{\"type\": \"FeatureCollection\", \"features\": []}".to_string());
        }

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

    #[wasm_bindgen(js_name = updateCostFunction)]
    pub fn update_cost_function(&mut self, input: JsValue) -> Result<(), JsValue> {
        let cost: CostFunction = serde_wasm_bindgen::from_value(input)?;
        info!(
            "Changing cost to {}",
            serde_json::to_string(&cost).map_err(err_to_js)?
        );
        self.last_cost = cost;
        self.network
            .recalculate_cost(&mut self.last_cost)
            .map_err(err_to_js)?;
        // Doesn't touch the CH, because this is only meant to be used in the edge cost app, which
        // doesn't use the CH
        Ok(())
    }

    #[wasm_bindgen(js_name = getBounds)]
    pub fn get_bounds(&self) -> Vec<f64> {
        let mut bounds = vec![f64::MAX, f64::MAX, f64::MIN, f64::MIN];
        for i in self.network.intersections.values() {
            let (x, y) = i.to_degrees();
            bounds[0] = bounds[0].min(x);
            bounds[1] = bounds[1].min(y);
            bounds[2] = bounds[2].max(x);
            bounds[3] = bounds[3].max(y);
        }
        bounds
    }

    /// Returns GeoJSON with details per edge that can be used to explore cost functions
    #[wasm_bindgen(js_name = debugNetwork)]
    pub fn debug_network(&self) -> Result<String, JsValue> {
        self.network.to_debug_geojson().map_err(err_to_js)
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
