# Latent demand 

This repo is an example of how to... TODO

The overview:

- Create origin/destination requests
- Calculate routes, and count trips per road segment
- Filter for most popular segments that lack cycling 

References / inspiration:

- [Propensity to Cycle Tool](https://www.pct.bike) / [NPTScot](https://nptscot.github.io)
- [Ungap the Map](https://a-b-street.github.io/docs/software/ungap_the_map/tech_details.html#predict-impact)
- A [talk from March 2022](https://dabreegster.github.io/talks/tds_seminar_synthpop/slides.html)
- [GrowBike.net](https://growbike.net)
- [BikeOttawa LTS map](https://maps.bikeottawa.ca/lts/)

## Quick start

```shell
cargo run --release -- --network network.bin --config '{"requests":{"Generate":{"pattern":"FromEveryOriginToOneDestination","origins_path":"bedfordshire/origin_subpoints.geojson","destinations_path":"bedfordshire/destination_subpoints.geojson"}},"routing":"Custom"}'
```

## Requirements

- [odjitter](https://github.com/dabreegster/odjitter)
- Docker
- Rust
- ogr2ogr with [OSM support](https://gdal.org/drivers/vector/osm.html)
- jq

One goal is to write as little new code as possible; reuse existing tools that're good.

Perf requirements for all of England... OSRM needed (TODO retry with CH):

- 8G disk
- about 16 minutes
- peak RAM around 10GB

## Part 1: Generating origin/destination requests

Let's work in London and model people travelling from home to school. The origin will be the centroid of all buildings in OpenStreetMap, and the destination the centroid of all school buildings. There are problems with too little data (because OSM is missing many buildings) and too much (many buildings are not residential). Let's restrict the trips to within the same MSOA, and generate one for every person living there (according to 2011? census).

[odjitter](https://github.com/dabreegster/odjitter) needs 4 inputs:

- a GeoJSON with zones -- MSOAs for us
- a GeoJSON with origin subpoints -- the centroids of all buildings
- a GeoJSON with destination subpoints -- the centroids of all schools
- a CSV file specifying the number of trips between each zone

```shell
AREA=london
mkdir $AREA
cd $AREA
wget http://download.geofabrik.de/europe/great-britain/england/greater-london-latest.osm.pbf -O $AREA.osm.pbf
wget https://ramp0storage.blob.core.windows.net/nationaldata-v2/GIS/MSOA_2011_Pop20.geojson -O all_msoa_zones.geojson

# Check the bounding box of the osm.pbf from the header
osmium fileinfo $AREA.osm.pbf
# Clip zones to the area. Coordinates below are for London
ogr2ogr -f GeoJSON -spat -0.4792 51.2737 0.28346 51.70269 ${AREA}_zones.geojson all_msoa_zones.geojson

ogr2ogr -f GeoJSON -dialect sqlite -sql 'SELECT ST_Centroid(geometry) FROM multipolygons WHERE building IS NOT NULL' origin_subpoints.geojson $AREA.osm.pbf
ogr2ogr -f GeoJSON -dialect sqlite -sql 'SELECT ST_Centroid(geometry) FROM multipolygons WHERE amenity = "school"' destination_subpoints.geojson $AREA.osm.pbf

echo 'geo_code1,geo_code2,cycling' > od.csv
jq -r '.features | map(.properties | [.MSOA11CD, .MSOA11CD, .PopCount] | @csv) | join("\n")' ${AREA}_zones.geojson >> od.csv
```

Now we generate a GeoJSON file with the requests (LineStrings):

```shell
odjitter disaggregate \
  --od-csv-path od.csv \
  --zones-path ${AREA}_zones.geojson \
  --zone-name-key MSOA11CD \
  --output-path requests.geojson
```

## Part 2: Routing

### Preparing OSRM

This took a few minutes on my definitely-not-dying laptop:

```
mkdir osrm
cd osrm; ln -s ../$AREA.osm.pbf .; cd ..
docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-extract -p /opt/bicycle.lua /data/osrm/$AREA.osm.pbf
docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-contract /data/osrm/$AREA.osrm
docker run -t -i -p 5000:5000 -v "${PWD}:/data" osrm/osrm-backend osrm-routed /data/osrm/$AREA.osrm
```

Send a sample request:

```
curl 'http://localhost:5000/route/v1/driving/-0.24684906005859372,51.42955782907472;-0.3240966796875,51.51515248101072?overview=false&alternatives=false&steps=false&annotations=nodes'
```

### Calculating routes

```
cd ../aggregate_routes
cargo run --release ../$AREA/requests.geojson
```

Can't load a 1.8GB gj. In the short-term, try FGB instead, or get rid of the intermediate file now.

## Part 3: Using the output

Use [the overline viewer](https://github.com/acteng/overline/blob/master/rust/viewer.html) for now.

## TODO

- [ ] Rename repo
- [ ] Optionally remove direction
- [ ] Make a new, faster viewer

Future directions:

- Try other routing engines
- Play with the routing profiles
	- If we can improve any existing roads, we may want to just route based on distance and hilliness, ignoring existing comfort / one-wayness entirely
- Generate better input OD
	- Filter for origins, and weight them better (high-density vs low-density housing)
	- Weight destinations better
	- Send people beyond their own MSOA
