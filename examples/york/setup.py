import csv
import json
import sys

sys.path.append("..")

from utils import *

run(["mkdir", "-p", "input"])

download(
    url="http://download.geofabrik.de/europe/great-britain/england/north-yorkshire-latest.osm.pbf",
    outputFilename="input/north-yorkshire-latest.osm.pbf",
)
# Clip to York
# TODO This additionally requires osmium
run(
    [
        "osmium",
        "extract",
        "-b",
        # http://bboxfinder.com for the win
        "-1.18,53.90,-0.98,54.01",
        "input/north-yorkshire-latest.osm.pbf",
        "-o",
        "input/input.osm.pbf",
    ]
)

extractCentroids(pbfInput="input/input.osm.pbf", geojsonOutput="input/origins.geojson")
