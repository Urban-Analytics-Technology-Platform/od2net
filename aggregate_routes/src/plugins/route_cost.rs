use crate::config::CostFunction;
use crate::network::Edge;

pub fn edge_cost(edge: &Edge, cost: CostFunction) -> Option<usize> {
    // TODO Match the lts.ts definition
    if edge.tags.is("bicycle", "no")
        || edge.tags.is("highway", "motorway")
        || edge.tags.is("highway", "proposed")
    {
        return None;
    }

    let output = match cost {
        CostFunction::Distance => edge.length_meters,
        // TODO Reframe this to just penalize by LTS?
        CostFunction::AvoidMainRoads => {
            // TODO Match the LTS definitoins
            let penalty =
                if edge.tags.is("highway", "residential") || edge.tags.is("highway", "cycleway") {
                    1.0
                } else {
                    5.0
                };
            penalty * edge.length_meters
        }
    };
    Some(output.round() as usize)
}
