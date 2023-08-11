use crate::input::Filter;

pub fn should_skip_trip(filter: &Filter, total_distance_meters: f64) -> bool {
    if let Some(max) = filter.max_distance_meters {
        return total_distance_meters.round() as usize > max;
    }

    false
}
