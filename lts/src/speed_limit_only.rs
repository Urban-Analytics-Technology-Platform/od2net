use crate::{Tags, LTS};

pub fn speed_limit_only(tags: &Tags) -> (LTS, Vec<String>) {
    let msgs = vec!["Only looking at maxspeed".into()];
    // TODO Handle bicycle=no, on things like highway=footway

    // TODO Use parse::get_maxspeed_mph
    if let Some(mph) = tags
        .get("maxspeed")
        .and_then(|x| x.trim_end_matches(" mph").parse::<usize>().ok())
    {
        if mph <= 20 {
            return (LTS::LTS2, msgs);
        }
        if mph >= 40 {
            return (LTS::LTS4, msgs);
        }
        // Between 20 and 40
        return (LTS::LTS3, msgs);
    }

    /*if tags.is("highway", "residential") {
        return LTS::LTS1;
    }*/

    (LTS::NotAllowed, msgs)
}
