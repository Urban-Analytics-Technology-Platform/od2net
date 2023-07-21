# Latent demand 

This repo is an example of how to... TODO

The overview:

- Create origin/destination requests
- Calculate routes, and count trips per road segment
- Filter for most popular segments that lack cycling 

References / inspiration:
- [Propensity to Cycle Tool](https://www.pct.bike)
- [Ungap the Map](https://a-b-street.github.io/docs/software/ungap_the_map/tech_details.html#predict-impact)
- A [talk from March 2022](https://dabreegster.github.io/talks/tds_seminar_synthpop/slides.html)
- [GrowBike.net](https://growbike.net)

## Requirements

- [odjitter](https://github.com/dabreegster/odjitter)
- osmium
- Docker
- Node

## Part 1: Generating origin/destination requests

Let's work in London and model people travelling from home to school. The origin will be the centroid of all buildings in OpenStreetMap, and the destination the centroid of all school buildings. There are problems with too little data (because OSM is missing many buildings) and too much (many buildings are not residential), but improvements are an exercise for the reader!

```shell
# About 90MB
wget http://download.geofabrik.de/europe/great-britain/england/greater-london-latest.osm.pbf -O london.osm.pbf
# Select all building ways. 35MB, a few seconds to extract
osmium tags-filter london.osm.pbf w/building -o buildings.osm.pbf
# 
osmium export buildings.osm.pbf --geometry-types=polygon -o buildings.geojson
```

- TODO: We want to drop tags and transform to centroids
- Note centroid is overkill; any arbitrary point on the building would be fine. OSRM is going to snap it to a road anyway.
- All these intermediate serialization steps are pointless. Call odjitter as a library?

## Generating requests


```shell
odjitter disaggregate \
  --od-csv-path input/od.csv \
  --zones-path input/zones.geojson \
  --zone-name-key name \
  --output-path input/requests.geojson
```

## Preparing OSRM

This took a few minutes on my definitely-not-dying laptop:

```
mkdir -p osrm; cd osrm
docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-extract -p /opt/bicycle.lua /data/london.osm.pbf
docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-partition /data/london.osrm
docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-customize /data/london.osrm
docker run -t -i -p 5000:5000 -v "${PWD}:/data" osrm/osrm-backend osrm-routed --algorithm mld /data/london.osrm
```

Send a sample request:

```
curl 'http://localhost:5000/route/v1/driving/-0.24684906005859372,51.42955782907472;-0.3240966796875,51.51515248101072?overview=false&alternatives=false&steps=false&annotations=nodes'
```

## Calculating routes

Install node, `npm i`, then `npm run route`. View `output.geojson` to see segment-level counts for routes.

It'll be slow the first time you run this, generating a 300 MB file for London. Reasonably fast after that.

## Viewing the output

Use [the overline viewer](https://github.com/acteng/overline/blob/master/rust/viewer.html) for now.

## TODO

- [ ] Rename repo
- [ ] Optionally remove direction
- [ ] Make a new, faster viewer
- [ ] Generate more interesting requests
