# 2011 England home-to-work

- Origins: all buildings
- Destinations: all buildings (for now)
- Flows are "Driving a car or van" + "Passenger in a car or van"

Data sources:

- [wu03ew](https://www.nomisweb.co.uk/census/2011/wu03EW), UK MSOA to MSOA, about 2 million rows
  - (less for just England)
- MSOA zone geojson from ONS Geography

Notes / TODO:

- We could upfront filter places too far away without even routing, using the uptake model to find something near 0
- Try [wf02eq](https://www.nomisweb.co.uk/census/2011/wf02ew), UK output area to workplace zone, about 16 million rows

## Performance

...
