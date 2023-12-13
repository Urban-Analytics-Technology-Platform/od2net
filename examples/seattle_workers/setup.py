import csv
import json
import sys

sys.path.append("..")

from utils import *


def makeOSM():
    download(
        url="https://download.geofabrik.de/north-america/us/washington-latest.osm.pbf",
        outputFilename="input/washington-latest.osm.pbf",
    )
    # Clip to a bbox around Seattle
    run(
        [
            "osmium",
            "extract",
            "-b",
            "-122.45,47.39,-122.07,47.73",
            "input/washington-latest.osm.pbf",
            "-o",
            "input/input.osm.pbf",
        ]
    )


def makeOrigins():
    extractCentroids(
        pbfInput="input/input.osm.pbf", geojsonOutput="input/buildings.geojson"
    )


def makeDestinations():
    writeFixedOutputFile(
        "input/destinations.geojson",
        """{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"name":"hospital"},"geometry":{"coordinates":[-122.32407,47.6042786],"type":"Point"}}]}""",
    )


# Also returns valid zone names
def makeZones():
    zone_ids = set()
    # TODO Use popgetter API
    with open("/home/dabreegster/Downloads/seattle.geojson") as f1:
        gj = json.load(f1)
        for f in gj["features"]:
            props = {"name": str(f["properties"]["GEOID"])}
            f["properties"] = props
            zone_ids.add(props["name"])

        with open("input/zones.geojson", "w") as f2:
            f2.write(json.dumps(gj))
    return zone_ids


def makeOD(zone_ids):
    # TODO Use popgetter API
    with open("/home/dabreegster/Downloads/us_metrics.csv") as f1:
        with open("input/od.csv", "w") as f2:
            writer = csv.DictWriter(f2, fieldnames=["from", "to", "count"])
            writer.writeheader()

            # Just rename columns
            for row in csv.DictReader(f1):
                zone = str(row["GEO_ID"])
                if zone not in zone_ids:
                    continue
                writer.writerow(
                    {
                        "from": zone,
                        "to": "hospital",
                        "count": row["working_population"],
                    }
                )


if __name__ == "__main__":
    run(["mkdir", "-p", "input"])
    makeOSM()
    makeOrigins()
    makeDestinations()
    zone_ids = makeZones()
    makeOD(zone_ids)
