use crate::Tags;

pub fn get_num_lanes(tags: &Tags, msgs: &mut Vec<String>) -> usize {
    // TODO The original checks for semicolons in lanes. That's not on the wiki and pretty much
    // doesn't happen: https://taginfo.openstreetmap.org/keys/lanes#values
    if let Some(n) = tags.get("lanes").and_then(|x| x.parse::<usize>().ok()) {
        return n;
    }
    // TODO What about one-ways?
    msgs.push(format!("Missing or invalid 'lanes' tag, so assuming 2"));
    2
}

pub fn get_maxspeed_mph(tags: &Tags, msgs: &mut Vec<String>) -> usize {
    if let Some(maxspeed) = tags.get("maxspeed") {
        if let Ok(kmph) = maxspeed.parse::<f64>() {
            return (kmph * 0.621371).round() as usize;
        }
        if let Some(mph) = maxspeed
            .strip_suffix(" mph")
            .and_then(|x| x.parse::<usize>().ok())
        {
            return mph;
        }
    }
    // TODO Regional defaults
    let default = match tags.get("highway").unwrap().as_str() {
        "motorway" => 60,
        "primary" | "secondary" => 50,
        _ => 30,
    };
    msgs.push(format!(
        "Guessing max speed is {default} mph based on highway tag"
    ));
    default
}
