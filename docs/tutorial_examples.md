# Tutorial: Running pre-made examples

<https://od2net.org> includes some example networks. This tutorial will teach
you how to generate these yourself, then modify the cost function. This is a
good tutorial to complete before trying to create your own od2net input.

## Setup

I'm assuming some familiarity running commands in a terminal (through Mac or
Linux -- on Windows, you probably need
[WSL](https://learn.microsoft.com/en-us/windows/wsl/install)). You'll first
need to install a few things:

- [Rust](https://www.rust-lang.org/tools/install) (1.73 or newer)
- Python 3 (only standard library modules needed)
- ogr2ogr from [GDAL](https://gdal.org/download.html)
- [tippecanoe](https://github.com/felt/tippecanoe)
- [osmium](https://osmcode.org/osmium-tool/manual.html#installation)
- [git](https://git-scm.com/downloads)
- You should already have standard Unix tools `curl` and `gunzip` on your system

Instead of installing the Rust toolchain and compiling od2net, you can use pre-built Docker images. In commands below, instead of `cargo run --release config.json`, do this:

```shell
docker run -v $(pwd):/app ghcr.io/urban-analytics-technology-platform/od2net:main /app/config.json
```

## Running the Edinburgh example

Open your terminal and let's get started! First we'll clone the git repo and
navigate to the Edinburgh example.

```shell
git clone https://github.com/Urban-Analytics-Technology-Platform/od2net
cd od2net/examples/edinburgh
```

Next we'll prepare all of the input data. Take a look through
[setup.py](https://github.com/Urban-Analytics-Technology-Platform/od2net/blob/main/examples/edinburgh/setup.py)
to see the steps that will happen:

1.  Download the `osm.pbf` file containing Edinburgh from [Geofabrik](https://download.geofabrik.de/europe/great-britain/scotland.html)
2.  Use `osmium` to clip that file to a rectangle around just Edinburgh
3.  Use `ogr2ogr` to get the centroid of every building, storing as points in a GeoJSON file. These will be the origins and destinations of trips in od2net.
4.  Download a GeoJSON file with polygons representing census zones, courtesy [NPT](https://nptscot.github.io)
5.  Download (and transform) a CSV file that describes how many trips to calculate between every zone, also from NPT.

Depending on your network and computer, this should take just a few minutes:

```shell
python3 setup.py
```

You can take a look through the files in `input/` if you're curious. When you're ready, let's run od2net!

```shell
cargo run --release config.json
```

This will take a few minutes to compile od2net the first time. After that,
running for this example should take under a minute. You'll see some output
stats printed at the end, but don't worry, these will be also shown in the web
app.

## Exploring output with the web app

Let's see the results! Open <https://od2net.org> in your browser, "Choose
file", then load `od2net/examples/edinburgh/output/rnet.pmtiles`.

The thickness of lines shows the number of trips made using that road -- so thicker roads could be used by many cyclists, and need to have safe infrastructure. The four colors indicate that current LTS rating. You can hover on any road to see all the details: how many trips routed over this road, the reason why it has a certain LTS rating, and all of the OSM tags.

Do you notice any patterns?

## Changing the cost function

This example made a pretty bold assumption -- cyclists picked the shortest route. That means they'll treat a quiet residential street, segregated cycle lane, and a motorway all the same. The LTS colors show this. This means the results are only useful if we can improve **any** road, no matter the cost or political pushback. This is quite unrealistic in most places, so what if we instead want to see how the network looks if people avoid stressful roads? That'll tell us what existing quiet streets form useful routes and reveal any gaps where using quiet options isn't possible.

We'll edit [config.json](https://github.com/Urban-Analytics-Technology-Platform/od2net/blob/main/examples/edinburgh/config.json) to do this. This file refers to the input files we created with the Python script and describes the origin/destination requests we want to route for. It also describes how cyclists will choose routes. Let's edit that!

Open `config.json` in a text editor, and replace the `"cost": "Distance",` line with something like this:

```
"cost": {
  "ByLTS": {
    "lts1": 1.0,
    "lts2": 1.5,
    "lts3": 3.0,
    "lts4": 5.0
  }
},
```

These factors will be multiplied by the length of each road, and higher results are worse. So totally safe LTS 1 streets will just use distance as the cost. For LTS 2 routes, we'll penalize them by multiplying by 1.5. For high-stress LTS 4 roads, we'll penalize them by a factor of 5. That means a cyclist would choose to take a quiet LTS 1 route that's 5 times longer, just to avoid the LTS 4 road. You can tune these numbers, of course!

After you've edited the config, you can rerun od2net. But first you need to delete some intermediate files, since the cost function has changed.

```shell
rm -rf intermediate/ 
# Rename the first output file, so you can compare the two later
mv output/rnet.pmtiles output/rnet_direct.pmtiles
cargo run --release config.json
# Give the new output file a better name
mv output/rnet.pmtiles output/rnet_quiet.pmtiles
```

Now you can open `output/rnet_quiet.pmtiles` in the web app and compare results. What changes? Are there any gaps in the quiet network that might be cheap and quick to fix?

## Advanced: write your own cost function

That new cost function is still pretty simple. What if your definition of the "best route" doesn't just depend on distance and LTS, or what if you don't like the predefined LTS classification? You can write your own cost function from scratch in any language you like. od2net will give you all of the OSM tags for a road segment, its distance, and some other info, and you then return a number indicating the cost for routing over that road.

First let's tell od2net to use a custom command for cost. Edit `config.json` and change `cost` to this:

```
"cost": {
  "ExternalCommand": "python3 cost.py"
},
```

Then create a new file called `cost.py`, copying from a different [example cost.py](https://github.com/Urban-Analytics-Technology-Platform/od2net/blob/main/examples/york/cost.py). The program gets a JSON array of dictionaries in STDIN and needs to print a JSON array of integer numbers as a result. Each dictionary input gives you:

- `length_meters`
- `tags`, a JSON dictionary with the raw OSM tags
- `lts` as a number 0 to 4, with 0 representing "cyclists not allowed here"
- `nearby_amenities`, the number of shops that're closest to this road

The example does something very boring -- if [highway = residential](https://wiki.openstreetmap.org/wiki/Tag:highway%3Dresidential), just return length. Otherwise, multiply by 10, meaning all other roads will be **heavily** penalized. Write something more interesting here!

Some tips for writing your own cost function:

- You can use any language you want. It just needs to read STDIN and write to STDOUT in the way that was described.
- The output costs need to be rounded to integers.
- If you want to debug your script, you can't print to STDOUT, because od2net will try to parse this as the JSON number result. You can instead write to STDERR or to a temporary log file. Keep in mind od2net will call your script multiple times when running (there's some internal batch size set), so if you write to a file, name it something unique.

## Next steps

Now you can [setup od2net in a new place](tutorial_new_area.md), with your own origin/destination data!
