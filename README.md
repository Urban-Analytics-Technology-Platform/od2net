# od2net

WARNING: This repo is not ready for general use. The API and input/output formats will keep changing. There are big limitations not yet documented.

TODO: Write intro

- Create origin/destination requests
- Calculate routes, and count how many trips cross each road segment
- Filter for most popular segments that lack appropriate cycling infrastructure

## Customizing

The purpose of this tool is to generate route networks **quickly** for areas up to **national scale**. The different stages of the pipeline are all modular and customizable, and to get meaningful results, we'll need to improve the defaults in all of them.

### Generating requests

The pipeline needs a list of routing requests to run -- just a huge list of start/end coordinates. These should **not** be centroids of a large zone or anything like that.

Built-in options for `"requests"` currently include:

- `{ "Generate": { "pattern": "FromEveryOriginToOneDestination" } }`
  - One trip for every $AREA/origins.geojson to the first point in $AREA/destinations.geojson
- `{"Generate" : { "pattern": "FromEveryOriginToNearestDestination" } }`
  - One trip for every $AREA/origins.geojson to the nearest (as the crow flies) point in $AREA/destinations.geojson
- `{"Odjitter": { "path": "file.geojson" } }`
  - Use LineStrings from a GeoJSON file. You can use [odjitter](https://github.com/dabreegster/odjitter) to generate a number of trips between zones, picking specific weighted points from each zone.
  - Note this option is **not** recommended for performance. For an interesting amount of requests, the overhead of reading/writing this file full of requests and storing it in memory doesn't work.

Problems to solve include:

- OSM is missing buildings in many places
- Origins and destinations should be weighted, based on how many people live or work/shop/go to school/etc somewhere

### Routing

The pipeline currently has two methods for calculating a route, specified by `"routing"`:

- The built-in `"FastPaths"` option, which currently makes a number of very bad assumptions:
  - Every edge can be crossed either direction
  - When `"cost": "Distance", edge cost is just distance -- equivalent to calculating the most direct route, ignoring LTS
    - `"cost": "AvoidMainRoads"` uses hardcoded multipliers for main roads
  - No penalty for elevation gain
  - No handling for turn restrictions, penalties for crossing intersections, etc

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

## References / inspiration:

- [Propensity to Cycle Tool](https://www.pct.bike) / [NPTScot](https://nptscot.github.io)
- [Ungap the Map](https://a-b-street.github.io/docs/software/ungap_the_map/tech_details.html#predict-impact)
- A [talk from March 2022](https://dabreegster.github.io/talks/tds_seminar_synthpop/slides.html)
- [GrowBike.net](https://growbike.net)
- [BikeOttawa LTS map](https://maps.bikeottawa.ca/lts/)
- [nori](https://github.com/b-r-u/nori)
