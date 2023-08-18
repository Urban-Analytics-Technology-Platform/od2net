import sys

sys.path.append("..")

from utils import *

# Pick a sub-region from https://download.geofabrik.de/europe/great-britain/england.html
area = "greater-london"
# Pick from https://wiki.openstreetmap.org/wiki/Key:amenity
amenity = "school"

download(
    url=f"https://download.geofabrik.de/europe/great-britain/england/{area}-latest.osm.pbf",
    outputFilename="input.osm.pbf",
)

extractCentroids(pbfInput="input.osm.pbf", geojsonOutput="origins.geojson")
extractCentroids(
    pbfInput="input.osm.pbf",
    geojsonOutput="destinations.geojson",
    where=f"amenity = '{amenity}'",
)
