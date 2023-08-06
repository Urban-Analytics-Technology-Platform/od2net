# Inputs

``` {bash}
AREA=edinburgh
URL=http://download.geofabrik.de/europe/great-britain/scotland-latest.osm.pbf

# Fill the $AREA directory with 3 files: input.osm.pbf, origins.geojson, and destinations.geojson
mkdir $AREA
# Download only if needed
curl $URL -z $AREA/input.osm.pbf -o $AREA/input.osm.pbf
ogr2ogr -f GeoJSON -dialect sqlite -sql 'SELECT ST_Centroid(geometry) FROM multipolygons WHERE building IS NOT NULL' $AREA/origins.geojson $AREA/input.osm.pbf
ogr2ogr -f GeoJSON -dialect sqlite -sql 'SELECT ST_Centroid(geometry) FROM multipolygons WHERE amenity = "school"' $AREA/destinations.geojson $AREA/input.osm.pbf
```

Input zones and OD data:

``` {bash}
# Download zones for area
URL_ZONES=https://github.com/nptscot/npt/raw/main/data-raw/zones_edinburgh.geojson
curl $URL_ZONES -o $AREA/zones.geojson
URL_OD=https://github.com/nptscot/npt/raw/main/data-raw/od_subset.csv
curl $URL_OD -o $AREA/od.csv
```
