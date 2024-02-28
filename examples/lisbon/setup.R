# Aim: generate input data for od2net with R

# Python implementation (commented out)

# import csv
# import json

# from utils import *


# def makeOSM():
#     download(
#         url="https://download.geofabrik.de/europe/portugal-latest.osm.pbf",
#         outputFilename="input/portugal-latest.osm.pbf",
#     )
#     # Clip to Lisbon
#     run(
#         [
#             "osmium",
#             "extract",
#             "-b",
#             "-9.291687,38.673717,-9.080887,38.831685",
#             "input/portugal-latest.osm.pbf",
#             "-o",
#             "input/input.osm.pbf",
#         ]
#     )

check_and_change_directory = function(name) {
    if (!file.exists("input")) {
        if (dir.exists(name)) {
            message("Changing working directory to ", name)
            setwd(name)
        } else {
            stop("Please run this script from the root of the repository")
        }
    }
}

make_osm = function() {
    # Check you're in the right working directory and if not cd
    check_and_change_directory("examples/lisbon")
    # TODO: use osmextract to download the file?
    # TODO: new function check_and_download_file()?
    if (!file.exists("input/portugal-latest.osm.pbf")) {
      download.file(
          url = "https://download.geofabrik.de/europe/portugal-latest.osm.pbf",
          destfile = "input/portugal-latest.osm.pbf"
      )
    }
    # Clip to Lisbon:
    system(
        "osmium extract -b -9.291687,38.673717,-9.080887,38.831685 input/portugal-latest.osm.pbf -o input/input.osm.pbf --overwrite"
    )
}
make_osm()

# def makeElevation():
#     download(
#         url="https://assets.od2net.org/input/LisboaIST_10m_4326.tif",
#         outputFilename="input/LisboaIST_10m_4326.tif",
#     )

make_elevation = function() {
    # Check you're in the right working directory and if not cd
    check_and_change_directory("examples/lisbon")
    # Download the file
    if (!file.exists("input/LisboaIST_10m_4326.tif")) {
      download.file(
          url = "https://assets.od2net.org/input/LisboaIST_10m_4326.tif",
          destfile = "input/LisboaIST_10m_4326.tif"
      )
    }
}


# def makeOrigins():
#     # Use building centroids as origins
#     extractCentroids(
#         osmInput="input/input.osm.pbf", geojsonOutput="input/buildings.geojson"
#     )

# def extractCentroids(osmInput, geojsonOutput, where="building IS NOT NULL"):
#     run(
#         [
#             "ogr2ogr",
#             "-f",
#             "GeoJSON",
#             "-dialect",
#             "sqlite",
#             "-sql",
#             f"SELECT ST_Centroid(geometry) FROM multipolygons WHERE {where}",
#             geojsonOutput,
#             osmInput,
#         ]
#     )

extract_centroids = function(osmInput, geojsonOutput, where="building IS NOT NULL") {
    # Check you're in the right working directory and if not cd
    check_and_change_directory("examples/lisbon")
    # Use building centroids as origins
    system(
        paste(
            "ogr2ogr -f GeoJSON -dialect sqlite -sql",
            paste0(
                "SELECT ST_Centroid(geometry) FROM multipolygons WHERE ",
                where
            ),
            geojsonOutput,
            osmInput
        )
    )
}
make_origins = function() {
    extract_centroids(
        osmInput = "input/input.osm.pbf",
        geojsonOutput = "input/buildings.geojson"
    )
}


# def makeDestinations():
#     # Same as origins
#     pass


# def makeZones():
#     download(
#         url="https://github.com/U-Shift/biclar/releases/download/0.0.1/zones.geojson",
#         outputFilename="input/zones.geojson",
#     )

make_zones = function() {
    # Check you're in the right working directory and if not cd
    check_and_change_directory("examples/lisbon")
    # Download the file
    if (!file.exists("input/zones.geojson")) {
      download.file(
          url = "https://github.com/U-Shift/biclar/releases/download/0.0.1/zones.geojson",
          destfile = "input/zones.geojson"
        )
    }
}

# def makeOD():
#     download(
#         url="https://github.com/U-Shift/biclar/releases/download/0.0.1/od.csv",
#         outputFilename="input/od.csv",
#     )
make_od = function() {
    # Check you're in the right working directory and if not cd
    check_and_change_directory("examples/lisbon")
    # Download the file
    if (!file.exists("input/od.csv")) {
      download.file(
          url = "https://github.com/U-Shift/biclar/releases/download/0.0.1/od.csv",
          destfile = "input/od.csv"
        )
    }
}

main = function() {
    dir.create("input", showWarnings = FALSE)
    make_osm()
    make_elevation()
    make_origins()
    make_zones()
    make_od()
}


# if __name__ == "__main__":
#     checkDependencies()
#     run(["mkdir", "-p", "input"])
#     makeOSM()
#     makeElevation()
#     makeOrigins()
#     makeDestinations()
#     makeZones()
#     makeOD()
