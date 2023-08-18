import sys

sys.path.append("..")

from utils import *

# Pick a sub-region from https://download.geofabrik.de/europe/great-britain/england.html
area = "greater-london"
# Pick from https://wiki.openstreetmap.org/wiki/Key:amenity
amenity = "school"

run(["mkdir", "-p", "input"])

download(
    url=f"https://download.geofabrik.de/europe/great-britain/england/{area}-latest.osm.pbf",
    outputFilename="input/input.osm.pbf",
)

extractCentroids(pbfInput="input/input.osm.pbf", geojsonOutput="input/origins.geojson")
extractCentroids(
    pbfInput="input/input.osm.pbf",
    geojsonOutput="input/destinations.geojson",
    where=f"amenity = '{amenity}'",
)
