use crate::tags::Tags;

pub enum LTS {
    NotAllowed,
    LTS1,
    LTS2,
    LTS3,
    LTS4,
}

impl LTS {
    pub fn into_json(self) -> usize {
        match self {
            LTS::NotAllowed => 0,
            LTS::LTS1 => 1,
            LTS::LTS2 => 2,
            LTS::LTS3 => 3,
            LTS::LTS4 => 4,
        }
    }
}

pub fn placeholder(tags: Tags) -> LTS {
    // TODO Handle bicycle=no, on things like highway=footway

    if let Some(mph) = tags
        .get("maxspeed")
        .and_then(|x| x.trim_end_matches(" mph").parse::<usize>().ok())
    {
        if mph <= 20 {
            return LTS::LTS2;
        }
        if mph >= 40 {
            return LTS::LTS4;
        }
        // Between 20 and 40
        return LTS::LTS3;
    }

    /*if tags.is("highway", "residential") {
        return LTS::LTS1;
    }*/

    LTS::NotAllowed
}

// The below is adapted from https://raw.githubusercontent.com/BikeOttawa/stressmodel/master/stressmodel.js, MIT licensed

// A flow chart would explain this nicely
fn bike_ottawa_lts(tags: Tags) -> (LTS, Vec<String>) {
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

    /*const ibl = isBikeLane(way);
    if (ibl.isBikeLane) {
      return ibl.result;
    }
    const imt = isMixedTraffic(way);
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
        //result = bikeLaneAnalysisParkingPresent(way, message);
        todo!()
    } else {
        //result = bikeLaneAnalysisNoParking(way, message);
        todo!()
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
