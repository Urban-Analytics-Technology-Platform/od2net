import requests
import sys

sys.path.append("..")

from utils import *


def oneshot(bbox):
    run(["rm", "-rf", "input", "intermediate", "output"])
    run(["mkdir", "-p", "input"])

    extractOverpass(bbox)

    # Use building centroids as origins and destinations
    extractCentroids(
        osmInput="input/input.osm.xml", geojsonOutput="input/buildings.geojson"
    )

    run(["cargo", "run", "--release", "--", "config.json", "--no-output-pmtiles"])

    # TODO Do something with the output


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
    # Note the order is weird: south, west, north, east
    oneshot([40.73, -74.0, 40.74, -73.98])
