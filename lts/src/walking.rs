use crate::{parse, Tags, LTS};

/// Ported from
/// https://github.com/acteng/edge_level_walkability_function/blob/main/walking-lts-prototyping.ipynb
pub fn walking(tags: &Tags) -> (LTS, Vec<String>) {
    let mut msgs = Vec::new();

    let speed_mph = parse::get_maxspeed_mph(tags, &mut msgs);

    if !can_traverse_on_foot(tags) || is_sidewalk(tags) || tags.is("highway", "elevator") {
        return (LTS::NotAllowed, msgs);
    }

    if is_separate_footpath(tags) {
        (LTS::LTS1, msgs)
    } else if is_pleasant_road(tags, speed_mph) || tags.is("highway", "steps") {
        (LTS::LTS2, msgs)
    } else if !tags.is_any("highway", vec!["motorway", "motorway_link"])
        && is_quite_unpleasant_road(tags, speed_mph)
    {
        (LTS::LTS3, msgs)
    } else if tags.is_any(
        "highway",
        vec!["motorway", "motorway_link", "trunk", "trunk_link"],
    ) || speed_mph > 40
    {
        (LTS::LTS4, msgs)
    } else {
        // TODO What cases are these?
        (LTS::NotAllowed, msgs)
    }
}

fn can_traverse_on_foot(tags: &Tags) -> bool {
    if !tags.has("highway") || tags.is("foot", "no") {
        return false;
    }
    if tags.is("access", "no") && !tags.is_any("foot", vec!["yes", "designated", "permissive"]) {
        return false;
    }
    if tags.is_any("highway", vec!["motorway", "motorway_link", "proposed"]) {
        return false;
    }
    true
}

fn is_separate_footpath(tags: &Tags) -> bool {
    if tags.is_any("highway", vec!["pedestrian", "path", "living_street"]) {
        return true;
    }
    if tags.is("highway", "footway")
        && !tags.is_any("footway", vec!["crossing", "link", "traffic_island"])
    {
        return true;
    }
    if tags.is("highway", "cycleway") && tags.is("footway", "designated") {
        return true;
    }
    false
}

fn is_pleasant_road(tags: &Tags, speed_mph: usize) -> bool {
    if tags.is_any(
        "highway",
        vec![
            "service",
            "alley",
            "driveway",
            "parking_aisle",
            "residential",
            "bridleway",
            "corridor",
            "track",
            "tertiary",
        ],
    ) && speed_mph <= 20
    {
        return true;
    }
    if tags.is("highway", "footway")
        && tags.is_any("footway", vec!["crossing", "link", "traffic_island"])
    {
        return true;
    }
    false
}

// TODO Logic here seems wrong
fn is_quite_unpleasant_road(tags: &Tags, speed_mph: usize) -> bool {
    let big_highway_type = tags.is_any(
        "highway",
        vec!["trunk", "trunk_link", "primary", "primary_link"],
    );
    if !big_highway_type && speed_mph <= 40 {
        return true;
    }
    if big_highway_type && speed_mph <= 20 {
        return true;
    }
    false
}

fn is_sidewalk(tags: &Tags) -> bool {
    tags.is("highway", "footway") && tags.is("footway", "sidewalk")
}
