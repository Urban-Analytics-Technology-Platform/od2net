use std::collections::HashMap;

use geo::{LineString, Polygon};
use osm_reader::NodeID;

use lts::Tags;

use crate::network::Position;

pub fn get_polygon(
    tags: &Tags,
    nodes: &HashMap<NodeID, Position>,
    node_ids: &Vec<NodeID>,
) -> Option<Polygon> {
    if !is_greenspace(tags) {
        return None;
    }

    // TODO Only handles closed area ways. No multipolygons, relations, holes, etc.
    // TODO Keep in WGS84 right now
    let mut pts = Vec::new();
    for id in node_ids {
        pts.push(nodes[id].to_degrees());
    }
    Some(Polygon::new(LineString::from(pts), vec![]))
}

// TODO From A/B Street. Relax these greatly?
fn is_greenspace(tags: &Tags) -> bool {
    if tags.is_any("leisure", vec!["garden", "park", "golf_course"]) {
        return true;
    }

    if tags.is_any("natural", vec!["wood", "scrub"]) {
        return true;
    }

    if tags.is_any(
        "landuse",
        vec![
            "cemetery",
            "flowerbed",
            "forest",
            "grass",
            "meadow",
            "recreation_ground",
            "village_green",
        ],
    ) {
        return true;
    }

    if tags.is("amenity", "graveyard") {
        return true;
    }

    false
}
