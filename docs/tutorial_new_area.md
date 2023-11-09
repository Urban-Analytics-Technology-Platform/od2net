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

od2net needs to know where to calculate routes.

The simplest input here is literally a GeoJSON file with thousands (or millions) of LineStrings, saying where each route should start and end. (Those endpoints don't have to be exactly on the road; they'll snap to the nearest intersection.) I don't recommend using raw LineStrings as input, because they're large to store, slow to read, and because od2net can help generate these from different patterns.

Most of the patterns use a GeoJSON file to specify individual origins and destinations as points. These are where trips will begin or end, usually buildings where people live, go to work, school, shops, etc.

Then you can specify an [ODPattern](https://github.com/Urban-Analytics-Technology-Platform/od2net/blob/main/od2net/src/config.rs) in the `config.json` file:

- `BetweenZones` is the most common pattern. You describe zone polygons that divide up your area, usually based on some kind of census boundary. Then you describe how many trips go between these zones.
  - You have to create a `zones.geojson` file with polygons that have a `name` property.
  - You also need an `od.csv` file with three columns: `from`, `to`, and `count`. The first two must match the zone names.
  - od2net will pick specific points within zones from the origin and destination GeoJSON points. It'll randomly sample (with replacement -- the same point can be used many times).
- `FromEveryOriginToNearestDestination` creates one trip for every point in your origin GeoJSON file. It'll go to the nearest destination point, measured as straight-line distance.

The other patterns are niche and may be removed or simplified soon.

The OD data you use depends on what you want to calculate. Some ideas:

- If you want to understand how cyclists currently travel around, find some data with start and endpoints.
- Or to explore how to "mode shift" existing trips by car to potential new cycling trips, look for data about short driving trips that happen today.
- Your country/city might have census data describing flows of people between home and work.
- You could generate your own demand or activity model by using census data to figure out how many schoolchildren live in different areas, then assigning them to different schools.

## Step 3: Configuration

TODO

https://github.com/Urban-Analytics-Technology-Platform/od2net/blob/main/od2net/src/config.rs
config.json spec

## Step 4: Running od2net

## Step 5: Using the output
