use crate::{is_cycling_allowed, parse, Tags, LTS};

pub fn speed_limit_only(tags: &Tags) -> (LTS, Vec<String>) {
    let mut msgs = vec!["Only looking at maxspeed".into()];

    if !is_cycling_allowed(tags, &mut msgs) {
        return (LTS::NotAllowed, msgs);
    }

    let mph = parse::get_maxspeed_mph(tags, &mut msgs);
    if mph <= 20 {
        return (LTS::LTS2, msgs);
    }
    if mph >= 40 {
        return (LTS::LTS4, msgs);
    }
    // Between 20 and 40
    (LTS::LTS3, msgs)
}
