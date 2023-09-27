import csv
import json
import sys

sys.path.append("..")

from utils import *

run(["mkdir", "-p", "input"])

download(
    url="https://download.geofabrik.de/europe/great-britain/england/merseyside-latest.osm.pbf",
    outputFilename="input/input.osm.pbf",
)

extractCentroids(pbfInput="input/input.osm.pbf", geojsonOutput="input/buildings.geojson")

# You have to manually download the Geopackage from https://geoportal.statistics.gov.uk/datasets/ons::output-areas-dec-2011-boundaries-ew-bgc/explore and put Output_Areas_Dec_2011_Boundaries_EW_BGC_2022_7971430631129549274.gpkg in input/

# First convert the CRS
run(
    [
        "ogr2ogr" "-f",
        "GeoJSON",
        "input/all_oas.geojson",
        "input/Output_Areas_Dec_2011_Boundaries_EW_BGC_2022_7971430631129549274.gpkg",
        "-t_srs",
        "EPSG:4326",
    ]
)
# Then clip (thanks to bboxfinder.com)
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
        "input/all_oas.geojson",
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


# To figure out the WPZ containing Alder Hey Hospital, go to https://geoportal.statistics.gov.uk/datasets/ons::workplace-zones-december-2011-full-clipped-boundaries-in-england-and-wales-1/explore, find the hospital by zooming in or using search, then copy the wz11cd property.
target_wpz = "E33003019"

# You have to manually download WF01AEW_oa from https://wicid.ukdataservice.ac.uk/flowdata/cider/wicid/downloads.php, unzip, and put wf01aew_oa_v1.csv in input/
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
