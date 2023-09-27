import sys

sys.path.append("..")

from utils import *

run(["mkdir", "-p", "input"])

download(
    url=f"https://download.geofabrik.de/europe/great-britain/england/greater-london-latest.osm.pbf",
    outputFilename="input/input.osm.pbf",
)

extractCentroids(pbfInput="input/input.osm.pbf", geojsonOutput="input/buildings.geojson")
extractCentroids(
    pbfInput="input/input.osm.pbf",
    geojsonOutput="input/schools.geojson",
    where=f"amenity = 'amenity'",
)
