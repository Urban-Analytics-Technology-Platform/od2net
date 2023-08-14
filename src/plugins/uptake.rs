use crate::input::Uptake;

/// Given stats about a route, calculate its "uptake", between 0 and 1.
pub fn calculate_uptake(uptake: &Uptake, total_distance_meters: f64) -> f64 {
    // TODO Find a data source and calculate this
    let gradient = 0.0;

    match uptake {
        Uptake::Identity => 1.0,
        Uptake::CutoffMaxDistanceMeters(max) => {
            if total_distance_meters > *max {
                0.0
            } else {
                1.0
            }
        }
        Uptake::GovTargetPCT => pct_gov_target(total_distance_meters, gradient),
        Uptake::GoDutchPCT => pct_go_dutch(total_distance_meters, gradient),
    }
}

// Everything below from
// https://github.com/ITSLeeds/pct/blob/e630464efeaef539b18647b10745b863c9cd9948/R/uptake.R
// TODO Switch to 2020 variations

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
    let result = p.exp() / (1.0 + p.exp());
    if result < 0.0 || result > 1.0 {
        panic!("inverse_logit({p}) = {result}, which isn't between 0 and 1");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs_err::File;
    use std::io::Write;

    // Load the resulting CSV files with https://www.csvplot.com to manually check
    #[test]
    fn test_pct() {
        for (path, model) in [
            ("gov_target.csv", pct_gov_target as fn(f64, f64) -> f64),
            ("go_dutch.csv", pct_go_dutch),
        ] {
            let mut file = File::create(path).unwrap();
            writeln!(file, "distance_km,gradient_percent,pcycle").unwrap();
            for distance_km in 0..=50 {
                for gradient in 0..=5 {
                    let distance_meters = distance_km as f64 * 1000.0;
                    let gradient_percent = gradient as f64;
                    let pcycle = model(distance_meters, gradient_percent);
                    writeln!(file, "{distance_km},{gradient},{pcycle}").unwrap();
                }
            }
        }
    }
}
