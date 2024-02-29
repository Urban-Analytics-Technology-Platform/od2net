import csv
import json
import os.path
import sys

from utils import *


def makeOSM():
    download(
        url="https://download.geofabrik.de/europe/great-britain/england-latest.osm.pbf",
        outputFilename="input/input.osm.pbf",
    )


def makeOrigins():
    extractCentroids(
        osmInput="input/input.osm.pbf", geojsonOutput="input/buildings.geojson"
    )


def makeDestinations():
    # Same as origins
    pass


def makeZones():
    download(
        url="https://github.com/dabreegster/uk-boundaries/raw/main/2021_output_areas.geojson.gz",
        outputFilename="input/zones.geojson.gz",
    )
    run(["gunzip", "input/zones.geojson.gz"])

    with open("input/zones.geojson") as f1:
        gj = json.load(f1)
        gj["features"] = list(
            filter(lambda f: f["properties"]["OA21CD"][0] == "E", gj["features"])
        )
        for f in gj["features"]:
            props = {"name": f["properties"]["OA21CD"]}
            f["properties"] = props

        with open("input/zones.geojson", "w") as f2:
            f2.write(json.dumps(gj))


def makeOD():
    download(
        url="https://www.nomisweb.co.uk/output/census/2021/odwp01ew.zip",
        outputFilename="input/odwp01ew.zip",
    )
    run(["unzip", "input/odwp01ew.zip", "-d", "input"])
    with open("input/ODWP01EW_OA.csv") as f1:
        with open("input/od.csv", "w") as f2:
            writer = csv.DictWriter(f2, fieldnames=["from", "to", "count"])
            writer.writeheader()

            for row in csv.DictReader(f1):
                zone1 = row["Output Areas code"]
                zone2 = row["OA of workplace code"]
                if zone1[0] == "E" and zone2[0] == "E":
                    # Ideally there'd be a split by commute mode. This dataset
                    # includes work-from-home, but just include all trips.
                    count = int(row["Count"])
                    if count > 0:
                        writer.writerow(
                            {
                                "from": zone1,
                                "to": zone2,
                                "count": count,
                            }
                        )


if __name__ == "__main__":
    checkDependencies()
    run(["mkdir", "-p", "input"])
    makeOSM()
    makeOrigins()
    makeDestinations()
    makeZones()
    makeOD()
