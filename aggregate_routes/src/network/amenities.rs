use lts::Tags;

/// Determines if this OSM object should count as some kind of useful commercial amenity. Many
/// categories are excluded.
pub fn is_amenity(tags: &Tags) -> bool {
    // TODO Allowlist might be easier
    if tags.is_any(
        "amenity",
        vec![
            "atm",
            "bench",
            "bicycle_parking",
            "bicycle_rental",
            "bicycle_repair_station",
            "car_rental",
            "car_sharing",
            "car_wash",
            "charging_station",
            "dog_litter_box",
            "drinking_water",
            "fuel",
            "grit_bin",
            "housing_office",
            "motorcycle_parking",
            "parcel_locker",
            "parking",
            "parking_entrance",
            "parking_meter",
            "parking_space",
            "post_box",
            "public_bookcase",
            "recycling",
            "taxi",
            "telephone",
            "toilets",
            "vending_machine",
            "waste_basket",
            "waste_disposal",
        ],
    ) {
        return false;
    }

    tags.has("amenity") || tags.has("shop")
}
