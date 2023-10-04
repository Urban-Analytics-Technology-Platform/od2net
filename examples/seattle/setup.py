import csv
import json
import sys

sys.path.append("..")

from utils import *


def makeOSM():
    download(
        url="https://download.geofabrik.de/north-america/us/washington-latest.osm.pbf",
        outputFilename="input/input.osm.pbf",
    )
    # TODO Clip?


# Returns a dictionary from parcel ID to [longitude, latitude]
def makeParcels():
    download(
        url="http://abstreet.s3-website.us-east-2.amazonaws.com/dev/data/input/us/seattle/parcels_urbansim.txt.gz",
        outputFilename="input/parcels_urbansim.csv.gz",
    )
    run(["gunzip", "input/parcels_urbansim.csv.gz"])

    # Convert to WGS84 and only keep parcel_id
    run(
        [
            "ogr2ogr",
            "-f",
            "CSV",
            "-s_srs",
            "ESRI:102748",
            "-t_srs",
            "EPSG:4326",
            "-select",
            "parcelid",
            "input/fixed_parcels.csv",
            "input/parcels_urbansim.csv",
            "-oo",
            "X_POSSIBLE_NAMES=xcoord_p",
            "-oo",
            "Y_POSSIBLE_NAMES=ycoord_p",
            "-lco",
            "GEOMETRY=AS_XY",
        ]
    )

    lookup = {}
    with open("input/fixed_parcels.csv") as f:
        for row in csv.DictReader(f):
            lookup[int(row["parcelid"])] = [
                round(float(row["X"]), 6),
                round(float(row["Y"]), 6),
            ]
    return lookup


def makeTrips(parcel_lookup):
    download(
        url="http://abstreet.s3-website.us-east-2.amazonaws.com/dev/data/input/us/seattle/trips_2014.csv.gz",
        outputFilename="input/trips_2014.csv.gz",
    )
    run(["gunzip", "input/trips_2014.csv.gz"])

    with open("input/trips_2014.csv") as f1:
        with open("input/trips.geojson", "w") as f2:
            f2.write("""{"type": "FeatureCollection", "features": [\n""")
            first = True
            for row in csv.DictReader(f1):
                # TODO Filter by mode?
                feature = {
                    "type": "Feature",
                    "geometry": {
                        "type": "LineString",
                        "coordinates": [
                            parcel_lookup[int(float(row["opcl"]))],
                            parcel_lookup[int(float(row["dpcl"]))],
                        ],
                    },
                }
                if first:
                    first = False
                else:
                    f2.write(",\n")
                f2.write(json.dumps(feature))
            f2.write("]}")


if __name__ == "__main__":
    run(["mkdir", "-p", "input"])
    makeOSM()
    parcels = makeParcels()
    makeTrips(parcels)
