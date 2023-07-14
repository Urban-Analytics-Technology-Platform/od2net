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

```
wget http://download.geofabrik.de/europe/great-britain/england/greater-london-latest.osm.pbf
```
