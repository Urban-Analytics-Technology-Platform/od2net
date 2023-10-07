use crate::{bike_ottawa, Tags, LTS};

#[test]
fn test_bike_ottawa() {
    // Use osm_unit_test_tool.html (open the file in your browser) to help generate
    for (way_id, input, expected_lts) in vec![(
        170171587,
        vec!["bicycle=yes", "foot=yes", "highway=footway"],
        LTS::LTS1,
    )] {
        let mut tags = Tags::new();
        for kv in input {
            let parts = kv.split("=").collect::<Vec<_>>();
            tags.insert(parts[0], parts[1]);
        }
        let (actual_lts, _) = bike_ottawa(&tags);
        if actual_lts != expected_lts {
            panic!(
                "For http://openstreetmap.org/way/{way_id}, got {:?} but expected {:?}",
                actual_lts, expected_lts
            );
        }
    }
}
