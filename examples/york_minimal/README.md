

The setup information is contained within the `setup.py` file, which
generates minimal input files.

``` bash
python setup.py
```

We’ll get a sample of 2 schools in York (York High School and Huntington
School) using the `osmextract` package.

``` r
library(osmextract)
```

    Data (c) OpenStreetMap contributors, ODbL 1.0. https://www.openstreetmap.org/copyright.
    Check the package website, https://docs.ropensci.org/osmextract/, for more details.

``` r
q = "SELECT * FROM multipolygons WHERE amenity='school'"
schools_york = osmextract::oe_get("York", query = q, extra_tags = "amenity")
```

    No exact match found for place = York and provider = geofabrik. Best match is Corse. 
    Checking the other providers.

    No exact match found in any OSM provider data. Searching for the location online.

    The input place was matched with North Yorkshire. 

    The chosen file was already detected in the download directory. Skip downloading.

    The corresponding gpkg file was already detected. Skip vectortranslate operations.

    Reading query `SELECT * FROM multipolygons WHERE amenity='school''
    from data source `/home/robin/data/osm/geofabrik_north-yorkshire-latest.gpkg' 
      using driver `GPKG'
    Simple feature collection with 603 features and 25 fields
    Geometry type: MULTIPOLYGON
    Dimension:     XY
    Bounding box:  xmin: -2.546044 ymin: 53.6425 xmax: -0.2912398 ymax: 54.61681
    Geodetic CRS:  WGS 84

``` r
# schools_york$name
destinations = dplyr::filter(
    schools_york,
    name %in% c("York High School", "Huntington School")
) |>
  dplyr::select(name, everything())
destinations$name
```

    [1] "York High School"  "Huntington School"

``` r
# Remove columns that only contain NA:
destinations = destinations[, colSums(is.na(destinations)) < nrow(destinations)]
destinations = sf::st_centroid(destinations)
```

    Warning: st_centroid assumes attributes are constant over geometries

``` r
sf::write_sf(destinations, "input/destinations.geojson", delete_dsn = TRUE)
```

We’ll also create a sample of subpoints in York, taking 3 random points
from each zone.

``` r
zones = sf::st_read("input/zones.geojson")
```

    Reading layer `zones' from data source 
      `/home/robin/github/Urban-Analytics-Technology-Platform/od2net/examples/york_minimal/input/zones.geojson' 
      using driver `GeoJSON'
    Simple feature collection with 3 features and 1 field
    Geometry type: POLYGON
    Dimension:     XY
    Bounding box:  xmin: -1.146752 ymin: 53.92474 xmax: -1.025942 ymax: 54.01074
    Geodetic CRS:  WGS 84

``` r
set.seed(123)
subpoints = sf::st_sample(zones, size = rep(3, nrow(zones))) |>
    sf::st_sf()
# Let's add provide the subpoints with values representing their importance:
subpoints$size = runif(nrow(subpoints), 1, 10) |>
    round(1)
sf::write_sf(subpoints, "input/subpoints.geojson", delete_dsn = TRUE)
```

We can visualise these as follows:

``` python
import geopandas as gpd
import pandas as pd
zones = gpd.read_file("input/zones.geojson")
destinations = gpd.read_file("input/destinations.geojson")
subpoints = gpd.read_file("input/subpoints.geojson")
od = pd.read_csv("input/od.csv")
ax = zones.plot()
destinations.plot(ax=ax, color='red')
subpoints.plot(ax=ax, color='blue', markersize=subpoints['size'] * 3)
ax.set_title("Origins and Destinations")
```

![](README_files/figure-commonmark/origins_destinations_plot-1.png)

Let’s visualise the flows between the origins and destinations:

``` r
library(ggplot2)
od = readr::read_csv("input/od.csv")
```

    Rows: 6 Columns: 3
    ── Column specification ────────────────────────────────────────────────────────
    Delimiter: ","
    chr (2): from, to
    dbl (1): count

    ℹ Use `spec()` to retrieve the full column specification for this data.
    ℹ Specify the column types or set `show_col_types = FALSE` to quiet this message.

``` r
od_geo = od::od_to_sf(od, zones, destinations)
```

    0 origins with no match in zone ids
    0 destinations with no match in zone ids
     points not in od data removed.

``` r
ggplot() +
  geom_sf(data = zones, fill = "grey") +
  geom_sf(data = subpoints, aes(size = size), color = "blue") +
  geom_sf(data = destinations, color = "red") +
  geom_sf(data = od_geo, aes(size = count), color = "black")
```

![](README_files/figure-commonmark/flows_plot-3.png)

We can then run the od2net command as follows:

``` bash
docker run -v $(pwd):/app ghcr.io/urban-analytics-technology-platform/od2net:main /app/config.json
```
