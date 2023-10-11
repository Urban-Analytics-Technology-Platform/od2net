import csv
import json
import sys

sys.path.append("..")

from utils import *


def makeOSM():
    download(
        url="https://download.geofabrik.de/europe/great-britain/england/merseyside-latest.osm.pbf",
        outputFilename="input/input.osm.pbf",
    )


def makeOrigins():
    extractCentroids(
        pbfInput="input/input.osm.pbf", geojsonOutput="input/buildings.geojson"
    )


def makeDestinations():
    writeFixedOutputFile(
        "input/destinations.geojson",
        """{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"name":"hospital"},"geometry":{"coordinates":[-2.89492,53.41828],"type":"Point"}}]}""",
    )


# Also returns valid oa_ids
def makeZones():
    # This is a prebuilt version of 2011 output areas, converted to WGS84. You could also manually download the Geopackage from https://geoportal.statistics.gov.uk/datasets/ons::output-areas-dec-2011-boundaries-ew-bgc/explore and convert the CRS.
    download(
        url="http://od2net.s3-website.eu-west-2.amazonaws.com/input/2011_oas.geojson.gz",
        outputFilename="input/2011_oas.geojson.gz",
    )
    run(["gunzip", "input/2011_oas.geojson.gz"])

    # Clip to Edinburgh (thanks to bboxfinder.com)
    run(
        [
            "ogr2ogr",
            "-f",
            "GeoJSON",
            "input/clipped_oas.geojson",
            "-clipsrc",
            "-3.290405",
            "53.291900",
            "-2.532349",
            "53.512960",
            "input/2011_oas.geojson",
        ]
    )

    # Then manually fix properties
    oa_ids = set()
    with open("input/clipped_oas.geojson") as f1:
        gj = json.load(f1)
        for f in gj["features"]:
            props = {"name": f["properties"]["OA11CD"]}
            f["properties"] = props
            oa_ids.add(props["name"])

        with open("input/zones.geojson", "w") as f2:
            f2.write(json.dumps(gj))
    return oa_ids


def makeOD(oa_ids):
    # To figure out the WPZ containing Alder Hey Hospital, go to https://geoportal.statistics.gov.uk/datasets/ons::workplace-zones-december-2011-full-clipped-boundaries-in-england-and-wales-1/explore, find the hospital by zooming in or using search, then copy the wz11cd property.
    target_wpz = "E33003019"

    # This is a cached version of WF01AEW_oa from https://wicid.ukdataservice.ac.uk/flowdata/cider/wicid/downloads.php
    download(
        url="http://od2net.s3-website.eu-west-2.amazonaws.com/input/wf01aew_oa_v1.csv.gz",
        outputFilename="input/wf01aew_oa_v1.csv.gz",
    )
    run(["gunzip", "input/wf01aew_oa_v1.csv.gz"])

    # TODO Or in-place
    with open("input/wf01aew_oa_v1.csv") as f1:
        with open("input/od.csv", "w") as f2:
            writer = csv.DictWriter(f2, fieldnames=["from", "to", "count"])
            writer.writeheader()

            for row in csv.reader(f1):
                from_oa = row[0]
                # This dataset doesn't break down by mode
                count = int(row[2])

                if row[1] != target_wpz:
                    continue
                if from_oa not in oa_ids:
                    print(
                        f"Warning: skipping {count} trips from {from_oa}, because they're outside the clipped area around Liverpool"
                    )
                    continue

                writer.writerow(
                    {
                        "from": from_oa,
                        "to": "hospital",
                        "count": count,
                    }
                )


if __name__ == "__main__":
    run(["mkdir", "-p", "input"])
    makeOSM()
    makeOrigins()
    makeDestinations()
    oa_ids = makeZones()
    makeOD(oa_ids)
