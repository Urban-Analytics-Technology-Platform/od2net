use crate::{parse, Tags, LTS};

// The below is adapted from https://raw.githubusercontent.com/BikeOttawa/stressmodel/master/stressmodel.js, MIT licensed
// TODO Ask about differences: maxspeed parsing, highway=construction

// A flow chart would explain this nicely
pub fn bike_ottawa(tags: &Tags) -> (LTS, Vec<String>) {
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

    if let Some(lts) = is_mixed_traffic(&tags, &mut msgs) {
        return (lts, msgs);
    }

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
        Some(bike_lane_with_parking(tags, msgs))
    } else {
        Some(bike_lane_no_parking(tags, msgs))
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

fn bike_lane_with_parking(tags: &Tags, msgs: &mut Vec<String>) -> LTS {
    let is_residential = tags.is("highway", "residential");
    let num_lanes = parse::get_num_lanes(tags, msgs);
    let speed_mph = parse::get_maxspeed_mph(tags, msgs);

    // TODO The logic is very mutable. Can we simplify it?
    let mut lts = LTS::LTS1;

    if num_lanes >= 3 {
        msgs.push("3+ lanes and parking, so increasing LTS to 3".into());
        lts = LTS::LTS3;
    }

    // Width unknown, so some logic simplified

    if speed_mph < 25 || is_residential && lts < LTS::LTS2 {
        msgs.push("Based on speed and parking, increasing LTS to 2".into());
        lts = LTS::LTS2;
    }

    if speed_mph >= 25 {
        if speed_mph <= 30 {
            if lts < LTS::LTS2 {
                msgs.push("Based on speed and parking, increasing LTS to 2".into());
                lts = LTS::LTS2;
            }
        }
    } else if speed_mph < 40 {
        if lts < LTS::LTS3 {
            msgs.push("Based on speed and parking, increasing LTS to 3".into());
            lts = LTS::LTS3;
        }
    } else if lts < LTS::LTS4 {
        msgs.push("Based on speed and parking, increasing LTS to 4".into());
        lts = LTS::LTS4;
    }

    if !is_residential && lts < LTS::LTS3 {
        msgs.push("Increasing LTS to 3 because highway isn't residential".into());
        lts = LTS::LTS3;
    }

    if lts == LTS::LTS1 {
        msgs.push("LTS 1 because there's parking, the speed is low, there aren't many lanes, and it's a residential street".into());
    }

    lts
}

fn bike_lane_no_parking(tags: &Tags, msgs: &mut Vec<String>) -> LTS {
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

    lts
}

fn is_mixed_traffic(tags: &Tags, msgs: &mut Vec<String>) -> Option<LTS> {
    msgs.push("No bike lane or separated path; treating as mixed traffic".into());

    let is_residential = tags.is("highway", "residential");
    let num_lanes = parse::get_num_lanes(tags, msgs);
    let speed_mph = parse::get_maxspeed_mph(tags, msgs);

    if tags.is("motor_vehicle", "no") {
        msgs.push("motor_vehicle=no, so LTS 1".into());
        return Some(LTS::LTS1);
    }
    if tags.is_any("highway", vec!["steps", "pedestrian"]) {
        msgs.push(format!(
            "LTS 1 since highway={}",
            tags.get("highway").unwrap()
        ));
        return Some(LTS::LTS1);
    }
    if tags.is("highway", "footway") && tags.is("footway", "crossing") {
        msgs.push("LTS 2 because highway=footway and footway=crossing".into());
        return Some(LTS::LTS2);
    }
    if tags.is("highway", "service") && tags.is("service", "alley") {
        msgs.push("LTS 2 because highway=service and service=alley".into());
        return Some(LTS::LTS2);
    }
    if tags.is("highway", "track") {
        msgs.push("LTS 2 because highway=track".into());
        return Some(LTS::LTS2);
    }

    if speed_mph > 30 {
        msgs.push("LTS 4 because speed is over 30mph".into());
        return Some(LTS::LTS4);
    }

    if tags.is("highway", "service") {
        if tags.is_any("service", vec!["parking_aisle", "driveway"]) {
            msgs.push(format!(
                "LTS 2 since speed is under 30mph and service={}",
                tags.get("service").unwrap()
            ));
            return Some(LTS::LTS2);
        }
        if speed_mph <= 20 {
            msgs.push("LTS 2 because speed is under 20mph and highway=service".into());
            return Some(LTS::LTS2);
        }
    }

    if speed_mph <= 25 {
        if num_lanes <= 3 && is_residential {
            msgs.push(
                "LTS 2 since speed is under 25 mph, 3 or fewer lanes, and a residential street"
                    .into(),
            );
            return Some(LTS::LTS2);
        } else if num_lanes <= 3 {
            msgs.push(
                "LTS 3 since speed is under 25 mph, 3 or fewer lanes, on a non-residential street"
                    .into(),
            );
            return Some(LTS::LTS3);
        } else if num_lanes <= 5 {
            msgs.push("LTS 3 since speed is under 25 mph and 4 or 5 lanes".into());
            return Some(LTS::LTS3);
        } else {
            msgs.push("LTS 4 since speed is under 25 mph and there's more than 5 lanes".into());
            return Some(LTS::LTS4);
        }
    }

    if num_lanes < 3 && is_residential {
        msgs.push(
            "LTS 2 because speed is 25-30mph, 2 or less lanes, and a residential street.".into(),
        );
        return Some(LTS::LTS2);
    } else if num_lanes <= 3 {
        msgs.push(
            "LTS 3 because speed is 25-30mph, 3 or less lanes, and a non-residential street."
                .into(),
        );
        return Some(LTS::LTS3);
    } else {
        msgs.push("LTS 4 because there are more than 3 lanes.".into());
        return Some(LTS::LTS4);
    }
}
