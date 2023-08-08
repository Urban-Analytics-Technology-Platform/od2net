# Latent demand (name TBD)

WARNING: This repo is not ready for use.

TODO: Write intro

- Create origin/destination requests
- Calculate routes, and count how many trips cross each road segment
- Filter for most popular segments that lack appropriate cycling infrastructure

## Quick start

### Setup

You'll need:

- Rust (1.71 or newer)
- ogr2ogr with [OSM support](https://gdal.org/drivers/vector/osm.html)
- Node (at least v18, to run the web app)
- About X disk, Y RAM, and Z minutes to run

First let's create input data. Let's explore routes starting from any building in Cornwall and going to schools.

```shell
AREA=cornwall
URL=http://download.geofabrik.de/europe/great-britain/england/cornwall-latest.osm.pbf

# Fill the $AREA directory with 3 files: input.osm.pbf, origins.geojson, and destinations.geojson
mkdir $AREA
curl $URL -o $AREA/input.osm.pbf
ogr2ogr -f GeoJSON -dialect sqlite -sql 'SELECT ST_Centroid(geometry) FROM multipolygons WHERE building IS NOT NULL' $AREA/origins.geojson $AREA/input.osm.pbf
ogr2ogr -f GeoJSON -dialect sqlite -sql 'SELECT ST_Centroid(geometry) FROM multipolygons WHERE amenity = "school"' $AREA/destinations.geojson $AREA/input.osm.pbf
```

### Run the pipeline

Then run the pipeline, routing from every single building to the nearest school. Or to see a much more clear pattern in the output, use `FromEveryOriginToOneDestination` to go from every building to one arbitrary school.

```shell
cargo run --release -- '{"directory":"'"$AREA"'","requests":{"Generate":{"pattern":"FromEveryOriginToNearestDestination"}},"routing":{"FastPaths":{"cost":"Distance"}},"filter":{}}'
```

It'll be slow the first time you run (compiling the tool, parsing OSM data, and building a contraction hierarchy). Subsequent runs will be faster.

### View the output

Let's see which roads are most used. It's easiest to use the deployed web app at <https://dabreegster.github.io/routing-engines/>. Load `$AREA/output.geojson` here.

Or you can build and run the web app yourself:

```shell
cd viewer
# You need to install dependencies the first time
npm i
npm run dev
```

Then open <http://localhost:5173/routing-engines/> (or whatever npm says) in your browser.

The Level of Traffic Stress definitions shown come from [BikeOttawa](https://maps.bikeottawa.ca/lts/).

## Customizing

The purpose of this tool is to generate route networks **quickly** for areas up to **national scale**. The different stages of the pipeline are all modular and customizable, and to get meaningful results, we'll need to improve the defaults in all of them.

### Generating requests

The pipeline needs a list of routing requests to run -- just a huge list of start/end coordinates. These should **not** be centroids of a large zone or anything like that.

Built-in options currently include:

- `"requests":{"Generate":{"pattern":"FromEveryOriginToOneDestination"}}`
  - One trip for every $AREA/origins.geojson to the first point in $AREA/destinations.geojson
- `"requests":{"Generate":{"pattern":"FromEveryOriginToNearestDestination"}}`
  - One trip for every $AREA/origins.geojson to the nearest (as the crow flies) point in $AREA/destinations.geojson
- `"requests":{"Odjitter":{"path":"file.geojson"}}`
  - Use LineStrings from a GeoJSON file. You can use [odjitter](https://github.com/dabreegster/odjitter) to generate a number of trips between zones, picking specific weighted points from each zone.
  - Note this option is **not** recommended for performance. For an interesting amount of requests, the overhead of reading/writing this file full of requests and storing it in memory doesn't work.

Problems to solve include:

- OSM is missing buildings in many places
- Origins and destinations should be weighted, based on how many people live or work/shop/go to school/etc somewhere

### Routing

The pipeline currently has two methods for calculating a route:

- The built-in `"routing":"FastPaths"` option, which currently makes a number of very bad assumptions:
  - Every edge can be crossed either direction
  - When `"cost":"Distance", edge cost is just distance -- equivalent to calculating the most direct route, ignoring LTS
    - `"cost":"AvoidMainRoads"` uses hardcoded multipliers for main roads
  - No penalty for elevation gain
  - No handling for turn restrictions, penalties for crossing intersections, etc
- Calling a local instance of [OSRM](https://project-osrm.org)
  - The built-in routing profiles can be used and customized
  - The overhead of calling even a local instance of OSRM is tremendous, because we're going through HTTP and parsing JSON on both ends.

Note to use OSRM, you additionally need Docker and to prepare OSRM in your area:

```shell
cd $AREA
mkdir osrm
cd osrm
ln -s ../inputosm.pbf .
cd ..
docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-extract -p /opt/bicycle.lua /data/osrm/input.osm.pbf
docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-contract /data/osrm/input.osrm
```

And then run OSRM on prepared data:

```shell
docker run -t -i -p 5000:5000 -v "${PWD}:/data" osrm/osrm-backend osrm-routed /data/osrm/input.osrm

# To send a sample request:
curl 'http://localhost:5000/route/v1/driving/-0.24684906005859372,51.42955782907472;-0.3240966796875,51.51515248101072?overview=false&alternatives=false&steps=false&annotations=nodes'
```

### Filtering out routes

After we calculate a route, we may want to exclude it because it's too long or hilly to reasonably expect people to cycle, even if the route was made very safe.

To exclude all routes over 16km: pass this instead of the empty JSON for `"filter":{"max_distance_meters":16000}`

### Visualization

Open questions include:

- How to visualize the counts for different road segments?
  - Currently, direction is ignored
  - Currently, we linearly interpolate line width based on the min/max count of any edge. Optionally can clamp the upper limit to handle outliers.
- How to judge how suitable roads currently are for cycling
  - Currently using [BikeOttawa's Level of Traffic Stress code](https://maps.bikeottawa.ca/lts/)
- Could we also visualize how easy a road is to modify? (Looking for excess width, parking/turn lanes, etc)

## Performance

TODO: Measure disk, memory, and runtime requirements for different areas.

TODO: Note the custom routing only uses **one thread** right now. Way more gains coming soon.

This pipeline uses a number of techniques to achieve these results on a regular laptop:

- Avoid saving and loading huge intermediate files
  - This is why running odjitter as a separate step right now isn't recommended. We can instead lazily generate requests as the router needs work to do, if they don't fit in memory.
- Reduce overhead for calling the router
  - The cost of actually calculating a single route is absolutely tiny. We're calculating millions of routes. So, the overhead for communicating with the router and using the results **must** be tiny.
  - Calling even a local instance of OSRM over HTTP is very slow. We could try native bindings in the future.
  - Currently, using a Rust implementation of [contraction hierarchies](https://github.com/easbar/fast_paths/). Zero communication overhead.
- Minimize the results for each routing call
  - Prior approaches have gotten back GeoJSON LineStrings and OSM attributes covering the resulting route. This is incredibly expensive to deal with for many requests.
  - Prior approachs have tried to sum up counts for road segments by [using geometry to represent segments](https://github.com/acteng/overline). This is very slow, has potential floating point errors, can break near bridges/tunnels, etc.
  - Instead, we just ask the router for OSM node IDs (64-bit integers). An edge is just a pair of these. At the very last step when we're generating output GeoJSON to visualize, we can match these node IDs to objects in OSM and produce the same geometry and OSM attributes.

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
