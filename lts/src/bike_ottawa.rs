use crate::{parse, Tags, LTS};

// The below is adapted from https://raw.githubusercontent.com/BikeOttawa/stressmodel/master/stressmodel.js, MIT licensed
// TODO Ask about differences: maxspeed parsing, highway=construction

// A flow chart would explain this nicely
pub fn bike_ottawa(tags: Tags) -> (LTS, Vec<String>) {
    let mut msgs = Vec::new();

    if !is_biking_permitted(&tags, &mut msgs) {
        return (LTS::NotAllowed, msgs);
    }

    if is_separate_path(&tags, &mut msgs) {
        msgs.push("Separated paths are always LTS=1.".into());
        return (LTS::LTS1, msgs);
    }

    if let Some(lts) = bike_lane_case(&tags, &mut msgs) {
        return (lts, msgs);
    }

    // TODO
    /*const imt = isMixedTraffic(way);
    if (imt.isMixedTraffic) {
      return imt.result;
    }*/

    msgs.push("No categories matched".into());
    (LTS::NotAllowed, msgs)
}

fn is_biking_permitted(tags: &Tags, msgs: &mut Vec<String>) -> bool {
    if !tags.has("highway") && !tags.has("bicycle") {
        msgs.push("Way doesn't have a highway or bicycle tag".into());
        return false;
    }

    if tags.is("bicycle", "no") {
        msgs.push("Cycling not permitted due to bicycle=no".into());
        return false;
    }

    if tags.is("access", "no") {
        // TODO There are exceptions for bicycle
        msgs.push("Cycling not permitted due to access=no".into());
        return false;
    }

    if tags.is_any(
        "highway",
        vec!["motorway", "motorway_link", "proposed", "construction"],
    ) {
        msgs.push(format!(
            "Cycling not permitted due to highway={}",
            tags.get("highway").unwrap()
        ));
        return false;
    }

    if tags.is_any("highway", vec!["footway", "path"])
        && tags.is("footway", "sidewalk")
        && !tags.is("bicycle", "yes")
    {
        msgs.push(format!(
            "Cycling not permitted on highway={}, when footway=sidewalk and bicycle=yes is missing",
            tags.get("highway").unwrap()
        ));
        return false;
    }

    true
}

fn is_separate_path(tags: &Tags, msgs: &mut Vec<String>) -> bool {
    if tags.is_any("highway", vec!["cycleway", "path"]) {
        msgs.push(format!(
            "This way is a separated path because highway={}",
            tags.get("highway").unwrap()
        ));
        return true;
    }

    if tags.is("highway", "footway") && !tags.is("footway", "crossing") {
        msgs.push(
            "This way is a separated path because highway=footway and it's not a crossing".into(),
        );
        return true;
    }

    if let Some((key, value)) = tags.prefix_is_any("cycleway", vec!["track", "opposite_track"]) {
        msgs.push(format!(
            "This way is a separated path because {key}={value}"
        ));
        return true;
    }

    false
}

fn bike_lane_case(tags: &Tags, msgs: &mut Vec<String>) -> Option<LTS> {
    let mut has_lane = false;
    if let Some((key, value)) = tags.prefix_is_any(
        "cycleway",
        vec![
            "crossing",
            "lane",
            "left",
            "opposite",
            "opposite_lane",
            "right",
            "yes",
        ],
    ) {
        has_lane = true;
        msgs.push(format!("Way has a bike lane because {key}={value}"));
    }

    if tags.is("shoulder:access:bicycle", "yes") {
        msgs.push("Way has a bike lane because shoulder:access:bicycle=yes".into());
        has_lane = true;
    }

    if !has_lane {
        return None;
    }

    if has_parking_lane(tags, msgs) {
        None
        //bike_lane_with_parking(tags, msgs)
    } else {
        bike_lane_no_parking(tags, msgs)
    }
}

fn has_parking_lane(tags: &Tags, msgs: &mut Vec<String>) -> bool {
    if tags.is("parking", "yes") {
        msgs.push("Has parking lane because parking=yes".into());
        return true;
    }

    if let Some((key, value)) = tags.prefix_is_any(
        "parking:lane",
        vec!["parallel", "perpendicular", "diagonal", "yes", "marked"],
    ) {
        msgs.push(format!("Has parking lane because {key}={value}"));
        return true;
    }

    msgs.push("No parking lane".into());
    false
}

fn bike_lane_no_parking(tags: &Tags, msgs: &mut Vec<String>) -> Option<LTS> {
    let is_residential = tags.is("highway", "residential");
    let num_lanes = parse::get_num_lanes(tags, msgs);
    let speed_mph = parse::get_maxspeed_mph(tags, msgs);

    // TODO The logic is very mutable. Can we simplify it?
    let mut lts = LTS::LTS1;
    // TODO This is undefined
    let has_separating_median = false;
    if num_lanes == 3 && has_separating_median {
        msgs.push(format!(
            "3 lanes, separating median, and no parking, so at least LTS 2"
        ));
        lts = LTS::LTS2;
    }
    if num_lanes >= 3 {
        msgs.push(format!("3 or more lanes and no parking, so at least LTS 3"));
        lts = LTS::LTS3;
    }
    // The original has some cases based on width, but width is pretty much always unknown
    // Note some of the km/h values here are rounded/adjusted a bit from the original
    if speed_mph > 30 {
        if speed_mph < 40 {
            if lts <= LTS::LTS3 {
                msgs.push(format!(
                    "No parking, speed between 30-40 mph, so at least LTS 3"
                ));
                lts = LTS::LTS3;
            }
        } else if lts <= LTS::LTS4 {
            msgs.push(format!("No parking, speed over 40 mph, so at least LTS 4"));
            lts = LTS::LTS4;
        }
    }
    if !is_residential && lts < LTS::LTS3 {
        msgs.push(format!(
            "Non-residential road with a bike lane and no parking, so at least LTS 3"
        ));
        lts = LTS::LTS3;
    }

    if lts == LTS::LTS1 {
        msgs.push(format!(
            "No parking, speed under 30mph, highway=residential, and 2 lanes or less"
        ));
    }

    Some(lts)
}
