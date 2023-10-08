use crate::config::CostFunction;
use crate::network::Edge;

pub fn calculate_batch(cost: &CostFunction, input_batch: Vec<&Edge>) -> Vec<Option<usize>> {
    match cost {
        CostFunction::Distance => input_batch.into_iter().map(distance).collect(),
        CostFunction::AvoidMainRoads => input_batch.into_iter().map(avoid_main_roads).collect(),
    }
}

fn distance(edge: &Edge) -> Option<usize> {
    // TODO Match the lts.ts definition
    if edge.tags.is("bicycle", "no")
        || edge.tags.is("highway", "motorway")
        || edge.tags.is("highway", "proposed")
    {
        return None;
    }

    Some(edge.length_meters.round() as usize)
}

fn avoid_main_roads(edge: &Edge) -> Option<usize> {
    // TODO Match the lts.ts definition
    if edge.tags.is("bicycle", "no")
        || edge.tags.is("highway", "motorway")
        || edge.tags.is("highway", "proposed")
    {
        return None;
    }

    // TODO Reframe this to just penalize by LTS?
    let penalty = if edge.tags.is("highway", "residential") || edge.tags.is("highway", "cycleway") {
        1.0
    } else {
        5.0
    };

    Some((penalty * edge.length_meters).round() as usize)
}
