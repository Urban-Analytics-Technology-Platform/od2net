use crate::input::Uptake;

pub fn should_skip_trip(uptake: &Uptake, total_distance_meters: f64) -> bool {
    // TODO Find a data source and calculate this
    let gradient = 0.0;

    match uptake {
        Uptake::Identity => false,
        Uptake::CutoffMaxDistanceMeters(max) => total_distance_meters > *max,
        // TODO Right now we're interpreting probabilities less than 0.1 as "totally skip this
        // trip." Change the pipeline to add a fractional count for routes, instead of 0 or 1.
        Uptake::GovTargetPCT => pct_gov_target(total_distance_meters, gradient) < 0.1,
        Uptake::GoDutchPCT => pct_go_dutch(total_distance_meters, gradient) < 0.1,
    }
}

// Everything below from
// https://github.com/ITSLeeds/pct/blob/e630464efeaef539b18647b10745b863c9cd9948/R/uptake.R

// TODO What does gradient represent -- an average or total or something over the entire route?
// gradient should be in [0, 100]
// This returns [0.0, 1.0]
fn pct_gov_target(distance_meters: f64, gradient_percent: f64) -> f64 {
    let alpha = -3.959;
    let d1 = -0.5963;
    let d2 = 1.866;
    let d3 = 0.008050;
    let h1 = -0.2710;
    let i1 = 0.009394;
    let i2 = -0.05135;

    // TODO Why clamp to 30km?
    let distance_km = (distance_meters / 1000.0).min(30.0);

    let p = alpha
        + (d1 * distance_km)
        + (d2 * distance_km.sqrt())
        + (d3 * distance_km.powi(2))
        + (h1 * gradient_percent)
        + (i1 * distance_km * gradient_percent)
        + (i2 * distance_km.sqrt() * gradient_percent);
    inverse_logit(p)
}

fn pct_go_dutch(distance_meters: f64, gradient_percent: f64) -> f64 {
    let alpha = -3.959 + 2.523;
    let d1 = -0.5963 - 0.07626;
    let d2 = 1.866;
    let d3 = 0.008050;
    let h1 = -0.2710;
    let i1 = 0.009394;
    let i2 = -0.05135;

    // TODO Why clamp to 30km?
    let distance_km = (distance_meters / 1000.0).min(30.0);

    let p = alpha
        + (d1 * distance_km)
        + (d2 * distance_km.sqrt())
        + (d3 * distance_km.powi(2))
        + (h1 * gradient_percent)
        + (i1 * distance_km * gradient_percent)
        + (i2 * distance_km.sqrt() * gradient_percent);
    inverse_logit(p)
}

fn inverse_logit(p: f64) -> f64 {
    p.exp() / (1.0 + p.exp())
}
