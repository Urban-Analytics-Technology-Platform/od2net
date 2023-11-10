# Specifying origin/destination data

od2net needs to know where to calculate routes.

The simplest input here is literally a GeoJSON file with thousands (or millions) of LineStrings, saying where each route should start and end. (Those endpoints don't have to be exactly on the road; they'll snap to the nearest intersection.) I don't recommend using raw LineStrings as input, because they're large to store, slow to read, and because od2net can help generate these from different patterns.

Most of the patterns use a GeoJSON file to specify individual origins and destinations as points. These are where trips will begin or end, usually buildings where people live, go to work, school, shops, etc.

Then you can specify an [ODPattern](https://github.com/Urban-Analytics-Technology-Platform/od2net/blob/main/od2net/src/config.rs) in the `config.json` file:

- `BetweenZones` is the most common pattern. You describe zone polygons that divide up your area, usually based on some kind of census boundary. Then you describe how many trips go between these zones.
  - You have to create a `zones.geojson` file with polygons that have a `name` property.
  - You also need an `od.csv` file with three columns: `from`, `to`, and `count`. The first two must match the zone names.
  - od2net will pick specific points within zones from the origin and destination GeoJSON points. It'll randomly sample (with replacement -- the same point can be used many times).
- `FromEveryOriginToNearestDestination` creates one trip for every point in your origin GeoJSON file. It'll go to the nearest destination point, measured as straight-line distance.

The other patterns are niche and may be removed or simplified soon.

## Choosing this

The OD data you use depends on what you want to calculate. Some ideas:

- If you want to understand how cyclists currently travel around, find some data with start and endpoints.
- Or to explore how to "mode shift" existing trips by car to potential new cycling trips, look for data about short driving trips that happen today.
- Your country/city might have census data describing flows of people between home and work.
- You could generate your own demand or activity model by using census data to figure out how many schoolchildren live in different areas, then assigning them to different schools.

Avoid using the centroids of large zones. Instead, specify many points within each zone and let od2net sample from them.

## TODO old notes

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
