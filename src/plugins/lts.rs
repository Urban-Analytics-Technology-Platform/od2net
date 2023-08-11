use crate::tags::Tags;

// 1 suitable for kids, 4 high stress, 0 is unknown. Need to swap this out for something much
// better, and maybe make it directional!
pub fn placeholder(tags: Tags) -> usize {
    // TODO Handle bicycle=no, on things like highway=footway

    if let Some(mph) = tags
        .get("maxspeed")
        .and_then(|x| x.trim_end_matches(" mph").parse::<usize>().ok())
    {
        if mph <= 20 {
            return 2;
        }
        if mph >= 40 {
            return 4;
        }
        // Between 20 and 40
        return 3;
    }

    /*if tags.is("highway", "residential") {
        return 1;
    }*/

    0 // TODO unknown
}
