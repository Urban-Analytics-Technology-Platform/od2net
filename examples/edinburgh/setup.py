import csv
import json
import sys

sys.path.append("..")

from utils import *

# Pick a sub-region from https://download.geofabrik.de/europe/great-britain/england.html
area = "greater-london"
# Pick from https://wiki.openstreetmap.org/wiki/Key:amenity
amenity = "school"

run(["mkdir", "-p", "input"])

download(
    url="http://download.geofabrik.de/europe/great-britain/scotland-latest.osm.pbf",
    outputFilename="input/scotland-latest.osm.pbf",
)
# Clip to Edinburgh
# TODO This additionally requires osmium
run(
    [
        "osmium",
        "extract",
        "-b",
        "-3.35,55.85,-2.97,55.99",
        "input/scotland-latest.osm.pbf",
        "-o",
        "input/input.osm.pbf",
    ]
)

extractCentroids(pbfInput="input/input.osm.pbf", geojsonOutput="input/origins.geojson")
# Not every zone has a school, so for now, just use all buildings for origins
# and destinations
extractCentroids(
    pbfInput="input/input.osm.pbf", geojsonOutput="input/destinations.geojson"
)

download(
    url="https://raw.githubusercontent.com/nptscot/npt/main/data-raw/od_subset.csv",
    outputFilename="input/od_subset.csv",
)
# TODO Or in-place
with open("input/od_subset.csv") as f1:
    with open("input/od.csv", "w") as f2:
        writer = csv.DictWriter(f2, fieldnames=["from", "to", "count"])
        writer.writeheader()

        for row in csv.DictReader(f1):
            writer.writerow(
                {
                    "from": row["geo_code1"],
                    "to": row["geo_code2"],
                    # TODO Could limit to only some modes if desired
                    "count": row["all"],
                }
            )


download(
    url="https://raw.githubusercontent.com/nptscot/npt/main/data-raw/zones_edinburgh.geojson",
    outputFilename="input/zones_edinburgh.geojson",
)

with open("input/zones_edinburgh.geojson") as f1:
    gj = json.load(f1)
    for f in gj["features"]:
        props = {"name": f["properties"]["InterZone"]}
        f["properties"] = props

    with open("input/zones.geojson", "w") as f2:
        f2.write(json.dumps(gj))
