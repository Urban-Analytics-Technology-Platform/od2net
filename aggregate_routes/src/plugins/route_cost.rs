use crate::config::CostFunction;
use crate::network::Edge;

pub fn edge_cost(edge: &Edge, cost: CostFunction) -> Option<usize> {
    let tags = edge.cleaned_tags();

    // TODO Match the lts.ts definition
    if tags.is("bicycle", "no") || tags.is("highway", "motorway") || tags.is("highway", "proposed")
    {
        return None;
    }

    let output = match cost {
        CostFunction::Distance => edge.length_meters,
        // TODO Reframe this to just penalize by LTS?
        CostFunction::AvoidMainRoads => {
            // TODO Match the LTS definitoins
            let penalty = if tags.is("highway", "residential") || tags.is("highway", "cycleway") {
                1.0
            } else {
                5.0
            };
            penalty * edge.length_meters
        }
    };
    Some(output.round() as usize)
}
