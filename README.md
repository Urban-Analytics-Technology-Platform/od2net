# routing-engines

## Generating requests

Install [odjitter](https://github.com/dabreegster/odjitter)

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
wget http://download.geofabrik.de/europe/great-britain/england/greater-london-latest.osm.pbf -O london.osm.pbf
docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-extract -p /opt/bicycle.lua /data/london.osm.pbf
docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-partition /data/london.osrm
docker run -t -v "${PWD}:/data" osrm/osrm-backend osrm-customize /data/london.osrm
docker run -t -i -p 5000:5000 -v "${PWD}:/data" osrm/osrm-backend osrm-routed --algorithm mld /data/london.osrm
```

Send a sample request:

```
curl 'http://localhost:5000/route/v1/driving/-0.24684906005859372,51.42955782907472;-0.3240966796875,51.51515248101072?overview=false&alternatives=false&steps=false&annotations=nodes'
```
