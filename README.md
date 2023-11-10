# od2net

![Logo](viewer/assets/logo.png)

WARNING: This repo is not ready for general use. The API and input/output formats will keep changing. There are big limitations not yet documented.

od2net helps you turn *o*rigin/*d*estination data about where people travel into a cycling *net*work plan. You can use this to decide what streets to prioritize for safety improvements.

1.  You specify origins and destinations of trips that cyclists take today or of short trips taken by car
2.  You specify how cyclists would like to choose routes. Are fast/direct routes important, or sticking to quiet streets? Do you want to route near greenspace and shops?)
3.  od2net calculates routes very quickly, counting how many routes cross every street
4.  You use the web viewer to look at the most popular streets, filtering for streets that don't have good cycling infrastructure today

## How to use it

...

- set up with your own data, run on your computer (directly or with docker), and compute country-wide network with millions of trips in under an hour
- the quick setup route: clip a small area from OSM, use dummy OD data, tune cost function, and make route networks ending at a single point. interactive in your browser, no install required, get something in minutes

## Contributing

We'd love contributions of all sorts -- developers, designers, data scientists, and applying it somewhere new! Check out [GitHub Issues](https://github.com/Urban-Analytics-Technology-Platform/od2net/issues), file a new one, or email <dabreegster@gmail.com> to get started.

This project follows the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) and is Apache 2.0 licensed. See all [credits](docs/credits.md).







### Scoring route likelihood

Even if a route is perfectly safe, it might be unlikely somebody would use it just based on the total distance or hilliness. The configurable "uptake model" assigns a probability between 0 and 1 to every route, and this value is summed for each edge.

The possible values for `"uptake"`:

- `"identity"` -- every route counts as 1, equivalent to just counting every trip
- `{ "CutoffMaxDistanceMeters": 16000 }` -- trips over 16km are skipped entirely, otherwise they count as 1
- `"GovTargetPCT"` and `"GoDutchPCT"` are uptake models from the PCT, using distance and gradient (**currently hardcoded to 0**)

### Visualization

Open questions include:

- How to visualize the counts for different road segments?
  - Currently, direction is ignored
  - Currently, we linearly interpolate line width based on the min/max count of any edge. Optionally can clamp the upper limit to handle outliers.
- How to judge how suitable roads currently are for cycling
  - Currently using [BikeOttawa's Level of Traffic Stress code](https://maps.bikeottawa.ca/lts/)
- Could we also visualize how easy a road is to modify? (Looking for excess width, parking/turn lanes, etc)

## TODO

Some of the most important next steps:

- Check the validity of using rstar on WGS84 coordinates. I think since the distances are so small, it's OK to pretend we're in Euclidean space.
- Directly read in the graph from OSRM, so we don't have to reinvent the wheel for edge costs
- Explore a UI for comparing counts from two different runs (so we can compare OSRM with our own cost function, for example)
- Handle bidirectionality end-to-end -- track count in both directions, and produce a directed input graph

Longer-term:

- Explore edge bundling to deal with dual carriageways and similar. Or maybe this is just a UI problem
- Add an example of modifying the network to represent improvements
	- aka, Ungap the Map v2
- Make this entire thing easier to run -- generate configs using the web UI?
- Validate output counts against current numbers (switching to a quiet/balanced profile first!)
