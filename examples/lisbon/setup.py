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
    # TODO LisboaCOPERNICUS_clip.tif doesn't have any errors, but no data seems to get scraped
    # TODO LisboaIST_clip_r1.tif is apparently missing a TIF signature?
    download(
        url="https://github.com/U-Shift/Declives-RedeViaria/raw/main/raster/LisboaCOPERNICUS_clip.tif",
        outputFilename="input/LisboaCOPERNICUS_clip.tif",
    )


def makeOrigins():
    # Use building centroids as origins
    extractCentroids(
        osmInput="input/input.osm.pbf", geojsonOutput="input/buildings.geojson"
    )


def makeDestinations():
    # School centroids as destinations
    extractCentroids(
        osmInput="input/input.osm.pbf",
        geojsonOutput="input/schools.geojson",
        where=f"amenity = 'school'",
    )


if __name__ == "__main__":
    checkDependencies()
    run(["mkdir", "-p", "input"])
    makeOSM()
    #makeElevation()
    makeOrigins()
    makeDestinations()
