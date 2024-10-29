use crate::config::Uptake;

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
        Uptake::WalkToSchool {
            upper_limit,
            exponent,
        } => walk_to_school(total_distance_meters, *upper_limit, *exponent),
    }
}

// Everything below from
// https://github.com/ITSLeeds/pct/blob/e630464efeaef539b18647b10745b863c9cd9948/R/uptake.R
// TODO Switch to 2020 variations

// TODO What does gradient represent -- an average or total or something over the entire route?
// gradient should be in [0, 100]
// This returns [0.0, 1.0]
// alpha = -4.018,
// d1 = -0.6369,
// d2 = 1.988,
// d3 = 0.008775,
// h1 = -0.2555,
// h2 = -0.78,
// i1 = 0.02006,
// i2 = -0.1234,
// # logit (pcycle)= -4.018 +  (-0.6369 *  distance)  +
// #   (1.988  * distancesqrt)  +  (0.008775* distancesq) +
// #   (-0.2555* gradient) + (0.02006* distance*gradient) +
// #   (-0.1234* distancesqrt*gradient)
fn pct_gov_target(distance_meters: f64, gradient_percent: f64) -> f64 {
    let alpha = -4.018;
    let d1 = -0.6369;
    let d2 = 1.988;
    let d3 = 0.008775;
    let h1 = -0.2555;
    let h2 = -0.78;
    let i1 = 0.02006;
    let i2 = -0.1234;

    // gradient = gradient + h2
    let gradient_percent = gradient_percent + h2;

    // Clamp to 30km to prevent cycling potential increasing with distance
    // This happens because the polynomial function reaches a minimal value at around 30km
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

// gradient,
// alpha = -4.018 + 2.550,
// d1 = -0.6369 - 0.08036,
// d2 = 1.988,
// d3 = 0.008775,
// h1 = -0.2555,
// h2 = -0.78,
// i1 = 0.02006,
// i2 = -0.1234,
// verbose = FALSE) {
// distance_gradient = check_distance_gradient(distance, gradient, verbose)
// distance = distance_gradient$distance
// gradient = distance_gradient$gradient
// # Uptake formula from manual:
// # logit_pcycle = -4.018  +  (-0.6369  *  distance)  +  (1.988  *  distancesqrt) +
// # (0.008775  * distancesq) + (-0.2555 * gradient) + (0.02006 * distance*gradient) +
// # (-0.1234 * distancesqrt*gradient) + (2.550 * dutch) +  (-0.08036* dutch * distance) +
// # (0.05509* ebike * distance) + (-0.0002950* ebike * distancesq) + (0.1812* ebike * gradient)
// gradient = gradient + h2
fn pct_go_dutch(distance_meters: f64, gradient_percent: f64) -> f64 {
    let alpha = -4.018 + 2.550;
    let d1 = -0.6369 - 0.08036;
    let d2 = 1.988;
    let d3 = 0.008775;
    let h1 = -0.2555;
    let h2 = -0.78;
    let i1 = 0.02006;
    let i2 = -0.1234;

    let gradient_percent = gradient_percent + h2;
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

// Cycle to school
// alpha = -7.178,
// d1 = -1.870,
// d2 = 5.961,
// # d3 = -0.2401,
// h1 = -0.5290,
// h2 = -0.63
fn pct_gov_target_school(distance_meters: f64, gradient_percent: f64) -> f64 {
    let alpha = -7.178;
    let d1 = -1.870;
    let d2 = 5.961;
    let h1 = -0.5290;
    let h2 = -0.63;

    let gradient_percent = gradient_percent + h2;
    let distance_km = (distance_meters / 1000.0).min(30.0);

    let p = alpha + (d1 * distance_km) + (d2 * distance_km.sqrt()) + (h1 * gradient_percent);
    inverse_logit(p)
}

// alpha = -7.178 + 3.574,
// d1 = -1.870 + 0.3438,
// d2 = 5.961,
// h1 = -0.5290,
// h2 = -0.63,
fn pct_go_dutch_school(distance_meters: f64, gradient_percent: f64) -> f64 {
    let alpha = -7.178 + 3.574;
    let d1 = -1.870 + 0.3438;
    let d2 = 5.961;
    let h1 = -0.5290;
    let h2 = -0.63;

    let gradient_percent = gradient_percent + h2;
    let distance_km = (distance_meters / 1000.0).min(30.0);

    let p = alpha + (d1 * distance_km) + (d2 * distance_km.sqrt()) + (h1 * gradient_percent);
    inverse_logit(p)
}

fn inverse_logit(p: f64) -> f64 {
    let result = p.exp() / (1.0 + p.exp());
    if result < 0.0 || result > 1.0 {
        panic!("inverse_logit({p}) = {result}, which isn't between 0 and 1");
    }
    result
}

fn walk_to_school(distance_meters: f64, upper_limit: f64, exponent: f64) -> f64 {
    let distance_km = distance_meters / 1000.0;
    let p = (-distance_km * exponent).exp();
    p.min(upper_limit)
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
