use crate::Tags;

pub fn is_cycling_allowed(tags: &Tags, msgs: &mut Vec<String>) -> bool {
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
