# Exploring individual routes

This tutorial assumes you can [run od2net](tutorial_examples.md).

While developing OD data, a route cost function, an uptake model, etc, it can be helpful to debug individual routes produced by the tool. You can pass `--detailed-routes=50` to output 50 GeoJSON files with more detail. Here's an example in York:

```
cd examples/york
python3 setup.py
cargo run --release config.json --detailed-routes=50
```

See `output/` for the result.
