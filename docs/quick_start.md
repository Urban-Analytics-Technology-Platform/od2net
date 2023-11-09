# Quick start

There are 3 ways to use od2net:

1.  TODO: Just explore example output online -- goto XYZ
2.  Run an example
3.  Use od2net in your own project

TODO point to tutorials

### Docker

```shell
docker run -v $(pwd):/app ghcr.io/urban-analytics-technology-platform/od2net:main /app/config.json
```

### Exploring individual routes

While developing OD data, a route cost function, an uptake model, etc, it can be helpful to debug individual routes produced by the tool. You can pass `--detailed-routes=50` to output 50 GeoJSON files with more detail. Here's an example in York using Docker to run:

```
cd examples/york
python3 setup.py
docker run -v $(pwd):/app ghcr.io/urban-analytics-technology-platform/od2net:main /app/config.json --detailed-routes=50
```

See `output/` for the result.
