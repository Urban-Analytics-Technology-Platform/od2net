import subprocess

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


def extractCentroids(pbfInput, geojsonOutput, where="building IS NOT NULL"):
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
            pbfInput,
        ]
    )


def writeFixedOutputFile(path, contents):
    with open(path, "w") as f:
        f.write(contents)
