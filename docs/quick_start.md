# Quick start

There are 3 ways to use od2net:

1.  TODO: Just explore example output online -- goto XYZ
2.  Run an example
3.  Use od2net in your own project

## Running examples

The examples in this repo all use Python to download and prepare od2net input for different studies.

### Setup

You'll need:

- Rust (1.71 or newer) or Docker
- Python 3 (no external dependencies)
- Standard Unix tools like curl and gunzip
- ogr2ogr with [OSM support](https://gdal.org/drivers/vector/osm.html)
- [tippecanoe](https://github.com/felt/tippecanoe)
- [osmium](https://osmcode.org/osmium-tool/manual.html#installation)

### Run an example

```shell
cd examples/london
python3 setup.py
# Building the Rust binary
cargo run --release config.json
# Or use Docker instead
# docker run -v $(pwd):/app ghcr.io/dabreegster/od2net:main /app/config.json
```

Then go to <https://Urban-Analytics-Technology-Platform.github.io/od2net/> and load `examples/london/output/rnet.pmtiles`.

Some steps will be slow the first time (compiling Rust, parsing OpenStreetMap data, and building a contraction hierarchy). Subsequent runs will be faster.

### Exploring individual routes

While developing OD data, a route cost function, an uptake model, etc, it can be helpful to debug individual routes produced by the tool. You can pass `--detailed-routes=50` to output 50 GeoJSON files with more detail. Here's an example in York using Docker to run:

```
cd examples/york
python3 setup.py
docker run -v $(pwd):/app ghcr.io/dabreegster/od2net:main /app/config.json --detailed-routes=50
```

See `output/` for the result.
