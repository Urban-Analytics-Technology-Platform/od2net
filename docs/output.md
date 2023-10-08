# Output format

## Main output

The main mode of the tool outputs a GeoJSON FeatureCollection, with each LineString representing a road segment between two intersections. These LineStrings cover every segment in the imported network, and the order of points matches the direction of the original OpenStreetMap way. Each Feature has the following properties:

- If enabled, `osm_tags` is an object with string keys and values, representing the original OSM data for that way
- `way` is the OSM way ID of the road
- `node1` and `node2` are the OSM node IDs bounding this road segment. Intermediate nodes of a curvy way (of degree 2, with no other connecting roads) are not used.
- `count` represents the sum of trips along the segment. This is equal to the number of trips crossing the segment when the uptake model is "Identity", and something weighted for other uptake models.
- `cost` is the cost for crossing this segment for routing
- `lts` is the Level of Traffic Stress for the segment, based on the chosen configuration. `0` means not allowed, `1` is suitable for children, and `4` is high stress.
- `nearby_amenities` is the number of shops and amenities that're closest to this segment.

TODO: counts.csv

TODO: the pmtiles for rendering

## Detailed routes

If you call the tool with `--detailed_routes`, you'll get individual GeoJSON files, each representing one route. The route is broken into LineStrings representing each segment. The direction followed across the segment is indicated both by the order of points and `node1` and `node2`. The properties for each Feature are the same as above, except there's no `count`, since this is just a single route.

The top-level FeatureCollection has additional foreign members: `total_distance_meters` and `uptake`
