import json
import requests
import sys

sys.path.append("..")

from utils import *


# Note the order is weird: south, west, north, east
def oneshot(bbox):
    run(["rm", "-rf", "input", "intermediate", "output"])
    run(["mkdir", "-p", "input"])

    extractOverpass(bbox)

    # Use building centroids as origins
    extractCentroids(
        osmInput="input/input.osm.xml", geojsonOutput="input/buildings.geojson"
    )
    # Extract all the schools
    extractCentroids(
        osmInput="input/input.osm.xml",
        geojsonOutput="input/schools.geojson",
        where=f"amenity = 'school'",
    )

    run(["cargo", "run", "--release", "--", "config.json", "--no-output-pmtiles"])

    # Add a countPercent property to every feature
    with open("output/output.geojson") as file1:
        gj = json.load(file1)
        maxCount = 0
        for f in gj["features"]:
            if f["geometry"]["type"] == "LineString":
                maxCount = max(maxCount, f["properties"]["count"])

        for f in gj["features"]:
            if f["geometry"]["type"] == "LineString":
                f["properties"]["countPercent"] = f["properties"]["count"] / maxCount

        with open("output/output.geojson", "w") as file2:
            file2.write(json.dumps(gj))


def extractOverpass(bbox):
    # Construct a query to extract all XML data in the bbox. See
    # https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL
    query = "(nwr({0}); node(w)->.x; <;); out meta;".format(
        ",".join(str(x) for x in bbox)
    )
    url = f"https://overpass-api.de/api/interpreter?data={query}"
    print(f"Grabbing Overpass data: {url}")

    resp = requests.get(url)
    assert resp.status_code == 200
    with open("input/input.osm.xml", "wb") as file:
        file.write(resp.content)


if __name__ == "__main__":
    oneshot(
        [
            51.532052371713334,
            -0.11243456187589182,
            51.56382995062716,
            -0.06146649291614957,
        ]
    )
