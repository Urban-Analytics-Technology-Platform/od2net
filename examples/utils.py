import subprocess


def checkDependencies():
    deps = ["cargo", "tippecanoe", "curl", "ogr2ogr", "osmium", "gunzip"]
    # If you're using Docker, uncomment the line below
    # deps = ["curl", "ogr2ogr", "osmium", "gunzip"]

    for dep in deps:
        if subprocess.run(["which", dep], capture_output=True).returncode != 0:
            raise Exception(
                f"You're missing {dep}. See https://github.com/Urban-Analytics-Technology-Platform/od2net/blob/main/docs/tutorial_examples.md#setup for installation links. If you're using Docker, go edit utils.py and see the comment there."
            )


# Run a command, verifying success
def run(args):
    print(">", " ".join(args))
    subprocess.run(args, check=True)


def download(url, outputFilename):
    run(
        [
            "curl",
            url,
            "-o",
            outputFilename,
        ]
    )


def extractCentroids(osmInput, geojsonOutput, where="building IS NOT NULL"):
    run(
        [
            "ogr2ogr",
            "-f",
            "GeoJSON",
            "-dialect",
            "sqlite",
            "-sql",
            f"SELECT ST_Centroid(geometry) FROM multipolygons WHERE {where}",
            geojsonOutput,
            osmInput,
        ]
    )


def writeFixedOutputFile(path, contents):
    with open(path, "w") as f:
        f.write(contents)
