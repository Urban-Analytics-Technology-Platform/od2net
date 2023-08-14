# 2011 UK home-to-work

- Origins: all buildings
- Destinations: all buildings (for now)
- Flows are "Driving a car or van" + "Passenger in a car or van"

Data sources:

- [wu03ew](https://www.nomisweb.co.uk/census/2011/wu03EW), UK MSOA to MSOA, about 2 million rows
- MSOA zone geojson from ONS Geography

Notes / TODO:

- Just start with England?
- We could upfront filter places too far away without even routing, using the uptake model to find something near 0
- Try [wf02eq](https://www.nomisweb.co.uk/census/2011/wf02ew), UK output area to workplace zone, about 16 million rows

## Performance

...

## Setup

Using A/B Street mirrors for data sources right now, because the original sources are hard to script against.

Assumes you're in the current directory.

```bash
wget https://download.geofabrik.de/europe/great-britain/england-latest.osm.pbf

wget http://play.abstreet.org/dev/data/input/shared/wu03ew_v2.csv.gz
gunzip wu03ew_v2.csv.gz

wget http://play.abstreet.org/dev/data/input/shared/zones_core.geojson.gz
gunzip zones_core.geojson.gz
```
