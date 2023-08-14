import csv
import json
import subprocess

# Assumes you're in the current directory.


def run(args):
    print(">", " ".join(args))
    subprocess.run(args, check=True)


run(
    [
        "curl",
        "https://download.geofabrik.de/europe/great-britain/england-latest.osm.pbf",
        "-o",
        "input.osm.pbf",
    ]
)
run(
    [
        "ogr2ogr",
        "-f",
        "GeoJSON",
        "-dialect",
        "sqlite",
        "-sql",
        "SELECT ST_Centroid(geometry) FROM multipolygons WHERE building IS NOT NULL",
        "origins.geojson",
        "input.osm.pbf",
    ]
)
run(["ln", "-s", "origins.geojson", "destinations.geojson"])


# Using A/B Street mirrors for data sources right now, because the original sources are hard to script against.
run(
    [
        "curl",
        "http://play.abstreet.org/dev/data/input/shared/wu03ew_v2.csv.gz",
        "-o",
        "wu03ew_v2.csv.gz",
    ]
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

run(
    [
        "curl",
        "http://play.abstreet.org/dev/data/input/shared/zones_core.geojson.gz",
        "-o",
        "zones_core.geojson.gz",
    ]
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
