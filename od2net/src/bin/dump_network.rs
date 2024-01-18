use anyhow::{bail, Result};

/// This tool writes `network.geojson` with the OSM tags, LTS, and cost for every edge in a
/// network. No counts are calculated or included.
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Call this with a config.json file");
    }
    let config_path = &args[1];
    let config_json = fs_err::read_to_string(config_path)?;
    let mut config: od2net::config::InputConfig = match serde_json::from_str(&config_json) {
        Ok(config) => config,
        Err(err) => bail!("{config_path} is invalid: {err}"),
    };

    // Assume the config file is in the directory for the area
    let absolute_path = std::fs::canonicalize(config_path).unwrap();
    let directory = absolute_path.parent().unwrap().display();
    let osm_pbf_path = format!("{directory}/input/input.osm.pbf");

    let mut timer = od2net::timer::Timer::new();
    let network = od2net::network::Network::make_from_osm(
        &fs_err::read(osm_pbf_path)?,
        &config.lts,
        &mut config.cost,
        &mut timer,
    )?;

    fs_err::write("network.geojson", &network.to_debug_geojson()?)?;
    Ok(())
}
