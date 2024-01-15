import sys

sys.path.append("..")

from utils import *


def makeOSM():
    download(
        url=f"https://download.geofabrik.de/europe/great-britain/england/greater-london-latest.osm.pbf",
        outputFilename="input/input.osm.pbf",
    )


def makeOrigins():
    extractCentroids(
        osmInput="input/input.osm.pbf", geojsonOutput="input/buildings.geojson"
    )


def makeDestinations():
    extractCentroids(
        osmInput="input/input.osm.pbf",
        geojsonOutput="input/schools.geojson",
        where=f"amenity = 'school'",
    )


if __name__ == "__main__":
    checkDependencies()
    run(["mkdir", "-p", "input"])
    makeOSM()
    makeOrigins()
    makeDestinations()
