from utils import *


def makeOSM():
    download(
        url="http://download.geofabrik.de/europe/great-britain/england/north-yorkshire-latest.osm.pbf",
        outputFilename="input/north-yorkshire-latest.osm.pbf",
    )
    # Clip to York
    run(
        [
            "osmium",
            "extract",
            "-b",
            # http://bboxfinder.com for the win
            "-1.18,53.90,-0.98,54.01",
            "input/north-yorkshire-latest.osm.pbf",
            "-o",
            "input/input.osm.pbf",
        ]
    )


def makeOrigins():
    extractCentroids(
        osmInput="input/input.osm.pbf", geojsonOutput="input/buildings.geojson"
    )


def makeDestinations():
    writeFixedOutputFile(
        "input/destinations.geojson",
        """{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"name":"poppleton"},"geometry":{"coordinates":[-2.141025,53.988014],"type":"Point"}},{"type":"Feature","properties":{"name":"corndogs"},"geometry":{"coordinates":[-1.080074,53.959366],"type":"Point"}}]}""",
    )


def makeZones():
    writeFixedOutputFile(
        "input/zones.geojson",
        """{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"name":"center"},"geometry":{"coordinates":[[[-1.08285,53.970735],[-1.096017,53.958917],[-1.075591,53.947693],[-1.057528,53.967309],[-1.08285,53.970735]]],"type":"Polygon"}},{"type":"Feature","properties":{"name":"north"},"geometry":{"coordinates":[[[-1.094806,53.998733],[-1.068685,53.977605],[-1.025942,53.9965],[-1.056812,54.010736],[-1.094806,53.998733]]],"type":"Polygon"}},{"type":"Feature","properties":{"name":"south"},"geometry":{"coordinates":[[[-1.146752,53.957545],[-1.139312,53.924745],[-1.091661,53.9281],[-1.1067,53.956427],[-1.146752,53.957545]]],"type":"Polygon"}}]}""",
    )


def makeOD():
    writeFixedOutputFile(
        "input/od.csv",
        """from,to,count
south,poppleton,500
center,poppleton,100
north,poppleton,200
south,corndogs,800
center,corndogs,300
north,corndogs,600""",
    )


if __name__ == "__main__":
    checkDependencies()
    run(["mkdir", "-p", "input"])
    makeOSM()
    makeOrigins()
    makeDestinations()
    makeZones()
    makeOD()
