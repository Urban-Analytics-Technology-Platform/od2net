import csv
import json
import sys

sys.path.append("..")

from utils import *

download(
    url="https://download.geofabrik.de/europe/great-britain/england-latest.osm.pbf",
    outputFilename="input.osm.pbf",
)
extractCentroids(pbfInput="input.osm.pbf", geojsonOutput="origins.geojson")
run(["ln", "-s", "origins.geojson", "destinations.geojson"])


# Using A/B Street mirrors for data sources right now, because the original sources are hard to script against.
download(
    url="http://play.abstreet.org/dev/data/input/shared/wu03ew_v2.csv.gz",
    geojsonOutput="wu03ew_v2.csv.gz",
)
run(["gunzip", "wu03ew_v2.csv.gz"])
with open("wu03ew_v2.csv") as f1:
    with open("od.csv", "w") as f2:
        writer = csv.DictWriter(f2, fieldnames=["from", "to", "count"])
        writer.writeheader()

        for row in csv.DictReader(f1):
            zone1 = row["Area of residence"]
            zone2 = row["Area of workplace"]
            if zone1[0] == "E" and zone2[0] == "E":
                count = int(row["Driving a car or van"]) + int(
                    row["Passenger in a car or van"]
                )
                if count > 0:
                    writer.writerow(
                        {
                            "from": zone1,
                            "to": zone2,
                            "count": count,
                        }
                    )

download(
    url="http://play.abstreet.org/dev/data/input/shared/zones_core.geojson.gz",
    outputFilename="zones_core.geojson.gz",
)
run(["gunzip", "zones_core.geojson.gz"])
with open("zones_core.geojson") as f1:
    gj = json.load(f1)
    gj["features"] = list(
        filter(lambda f: f["properties"]["geo_code"][0] == "E", gj["features"])
    )
    for f in gj["features"]:
        props = {"name": f["properties"]["geo_code"]}
        f["properties"] = props

    with open("zones.geojson", "w") as f2:
        f2.write(json.dumps(gj))
