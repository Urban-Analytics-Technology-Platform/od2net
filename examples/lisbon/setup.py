import csv
import json

from utils import *


def makeOSM():
    download(
        url="https://download.geofabrik.de/europe/portugal-latest.osm.pbf",
        outputFilename="input/portugal-latest.osm.pbf",
    )
    # Clip to Lisbon
    run(
        [
            "osmium",
            "extract",
            "-b",
            "-9.291687,38.673717,-9.080887,38.831685",
            "input/portugal-latest.osm.pbf",
            "-o",
            "input/input.osm.pbf",
        ]
    )


def makeElevation():
    download(
        url="https://assets.od2net.org/input/LisboaIST_10m_4326.tif",
        outputFilename="input/LisboaIST_10m_4326.tif",
    )


def makeOrigins():
    # Use building centroids as origins
    extractCentroids(
        osmInput="input/input.osm.pbf", geojsonOutput="input/buildings.geojson"
    )


def makeDestinations():
    # Same as origins
    pass


def makeZones():
    download(
        url="https://github.com/U-Shift/biclar/releases/download/0.0.1/zones.geojson",
        outputFilename="input/zones.geojson",
    )


def makeOD():
    download(
        url="https://github.com/U-Shift/biclar/releases/download/0.0.1/od.csv",
        outputFilename="input/od.csv",
    )


if __name__ == "__main__":
    checkDependencies()
    run(["mkdir", "-p", "input"])
    makeOSM()
    makeElevation()
    makeOrigins()
    makeDestinations()
    makeZones()
    makeOD()
