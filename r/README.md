# Preparing OD data for network generation with od2net


This R package provides functions to prepare OD data for network
generation with the `od2net` tool, as illustrated in the example below.

## Installation

``` r
# Install pak if not already installed:
if (!requireNamespace("pak", quietly = TRUE)) {
  install.packages("pak")
}
pak::pkg_install("Urban-Analytics-Technology-Platform/od2net/r")
```

## Example

Imagine you want to generate a route network with values on the links
representing the number of pupils/parents on their way to school each
school day. The following code shows how to prepare the data for the
`od2net` tool.

The following functions could be useful when you’re preparing the data,
and show the kind of data preparation that is required.

<details>
<!-- TODO: consider adding these functions to the package at some stage? -->

``` r
# Aim: generate input data for od2net with R

#' Generate a 'zones.geojson' file
#' 
#' This function requires a zones file, e.g.
#' "https://raw.githubusercontent.com/nptscot/npt/main/data-raw/zones_edinburgh.geojson"
#' or a file on your computer.
#' It will generate a file in the input/ folder
#' 
#' @param file Location or URL of zones file
make_zones = function(file) {
  zones = sf::read_sf(file)[1]
  names(zones)[1] = "name"
  sf::write_sf(zones, "input/zones.geojson", delete_dsn = TRUE)
}

make_od = function() {
  od = readr::read_csv("https://raw.githubusercontent.com/nptscot/npt/main/data-raw/od_subset.csv")
  od = od |>
    dplyr::transmute(from = geo_code1, to = geo_code2, count = bicycle)
  readr::write_csv(od, "input/od.csv")
}
#' Get elevation data
#' 
#' This function downloads elevation data from a source such as
#' https://play.abstreet.org/dev/data/input/shared/elevation/UK-dem-50m-4326.tif.gz
#' or https://assets.od2net.org/input/LisboaIST_10m_4326.tif
#' 
#' @param url Full URL of the elevation dataset if available
#' @param file File name if hosted on a known site
#' @param base_url Base URL associated with the 'file' argument
#' 
make_elevation = function(
    url = NULL,
    file = "UK-dem-50m-4326.tif.gz",
    base_url = "https://play.abstreet.org/dev/data/input/shared/elevation/"
    ) {
  if (is.null(url)) {
    url = paste0(base_url, file)
  }
  is_gzip = grepl(pattern = "gz", url)
  # Download the file
    if (!file.exists("input/elevation.tif") && is_gzip) {
      download.file(
          url = url,
          destfile = "input/elevation.tif.gz"
      )
      R.utils::gunzip("input/elevation.tif.gz", destname = "input/elevation.tif")
    } else {
      download.file(
        url = url,
        destfile = "input/elevation.tif"
      )
    }
}
```

</details>

``` r
dir.create("input", showWarnings = FALSE)
# Get some zones from a URL:
uz = "https://github.com/acteng/netgen/raw/main/input/zones_york.geojson"
make_zones(uz)
sf::write_sf(zones, "input/zones.geojson", delete_dsn = TRUE)
make_osm(zones_file = "input/zones.geojson", output_file = "input/input.osm.pbf")
make_origins(
  osm_file = "input/input.osm.pbf",
  query = "SELECT osm_id FROM multipolygons WHERE building IS NOT NULL",
  output_file = "input/buildings.geojson"
)
make_elevation()
destinations = simodels::destinations_york # Provided in the R package
names(destinations)[1] = "name"
destinations = destinations[1]
class(destinations$name) = "character"
sf::write_sf(destinations, "input/destinations.geojson", delete_dsn = TRUE)
od_geo = sf::read_sf("https://github.com/acteng/netgen/releases/download/v0.1.0/res_output.geojson")
# Save the OD dataset:
od = od_geo |>
  sf::st_drop_geometry() |>
  dplyr::transmute(from = O, to = as.character(D), count = round(trips_modelled))
readr::write_csv(od, "input/od.csv", quote = "all")
```

Then create a config.json file, e.g. with the following content:

``` r
readLines("config.json") |> cat(sep = "\n")
```

    {
        "requests": {
          "description": "Test data for SchoolRoutes project.",
          "pattern": {
            "ZoneToPoint": {
              "zones_path": "zones.geojson",
              "destinations_path": "destinations.geojson",
              "csv_path": "od.csv",
               "origin_zone_centroid_fallback": false
            }
          },
          "origins_path": "buildings.geojson",
          "destinations_path": "destinations.geojson"
        },
        "cost": "Distance",
        "uptake": "Identity",
        "lts": "BikeOttawa",
        "elevation_geotiff": "elevation.tif"
    }

Then run the following code to generate the network:

Run the tool with Docker as follows:

``` bash
# On Linux:
sudo docker run -v $(pwd):/app ghcr.io/urban-analytics-technology-platform/od2net:main /app/config.json
# or in Windows:
sudo docker run -v ${pwd}:/app ghcr.io/urban-analytics-technology-platform/od2net:main /app/config.json
```

After that you should see something like the following in the output
folder:

``` r
fs::dir_tree("output")
```
