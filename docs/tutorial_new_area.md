# Tutorial: Running in a new area

This tutorial will guide you through using od2net in a new area. I'm assuming you've been through the [previous tutorial](tutorial_examples.md).

The overall process is:

1.  Create your study area
2.  Prepare origin/destination input
3.  Specify configuration to customize the route cost function, level of traffic stress, uptake model, etc.
4.  Run `odnet`
5.  Use the web viewer to explore the output, or write your own analysis to use the results

## Where to put your code

You can keep all of your work just on your own computer, or start your own git repo if you like. At minimum, it'll just be a few files, like any of the [examples](https://github.com/Urban-Analytics-Technology-Platform/od2net/tree/main/examples/london).

If you'd like to contribute your area to the od2net examples, please do! I'm not sure yet how many official examples I'll maintain long-term, but I'm happy to add many for now. Please fork the od2net repo, start a new branch with your addition, then [send a PR from your fork](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request-from-a-fork).

## Scripting

The od2net examples all use Python and rely on external tools like `osmium` and `ogr2ogr` to do lots of the heavy lifting. You're free to use R, Java, or any language you're comfortable with, and to use any dependencies that're helpful. You can also just create all of the inputs manually, though I'd encourage scripting so somebody else can reproduce your data science workflow.

## Step 1: Creating your study area

od2net needs an `osm.pbf` file as input. You can create this however you like -- the examples download a big file from Geofabrik, then use osmium to clip.

## Step 2: Preparing origin/destination input

See [here](config_od.md) to start your `config.json`.

## Step 3: Configuration

Now you need to specify:

- [edge cost function](config_cost.md)
- [uptake model](config_uptake.md)

See [the Rust definitions](https://github.com/Urban-Analytics-Technology-Platform/od2net/blob/main/od2net/src/config.rs) for all `config.json` options.

## Step 4: Running od2net

As before, either:

```shell
cargo run --release config.json
```

or:

```
docker run -v $(pwd):/app ghcr.io/urban-analytics-technology-platform/od2net:main /app/config.json
```

## Step 5: Using the output

Go to <https://od2net.org> to load `output/rnet.pmtiles`.
