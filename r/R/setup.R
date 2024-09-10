#' Get bounding box from zones
#'
#' This function reads a GeoJSON file containing zones and calculates the bounding box of the zones.
#' 
#' @param zones_file Path to the GeoJSON file containing zones. Default is 'input/zones.geojson'.
#'
#' @return A character string representing the bounding box in the format "xmin, ymin, xmax, ymax".
#' @export
getbbox_from_zones = function(zones_file = "input/zones.geojson") {
  zones = sf::st_read(zones_file)
  bbox = sf::st_bbox(zones)
  paste0(bbox, collapse = ",")
}

#' make_osm Function
#'
#' This function is used to download and extract OpenStreetMap (OSM) data based on specified zones.
#'
#' @param force_download A logical value indicating whether to force the download of OSM data even if it already exists. Default is \code{FALSE}.
#' @param zones_file The file path or name of the zones file in GeoJSON format. Default is \code{"input/zones.geojson"}.
#'
#' @return This function does not return any value. It downloads and extracts OSM data based on the specified zones.
#'
#' @examples
#' make_osm(force_download = TRUE, zones_file = "input/zones.geojson")
#'
#' @import sf
#' @import osmextract
#' @importFrom osmextract oe_match oe_download
#' @importFrom osmium getbbox_from_zones
#' @importFrom base system
#'
#' @export
make_osm = function(force_download = FALSE, zones_file = "input/zones.geojson") {  # Function coderl}
    file = "UK-dem-50m-4326.tif.gz",
    base_url = "https://play.abstreet.org/dev/data/input/shared/elevation/") {
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

make_origins = function() {
  buildings = sf::read_sf("input/input.osm.pbf", query = "SELECT osm_id FROM multipolygons WHERE building IS NOT NULL")
  use_sf = sf::sf_use_s2(FALSE)
  centroids = sf::st_centroid(buildings)
  sf::sf_use_s2(use_sf)
  sf::write_sf(centroids, "input/buildings.geojson", delete_dsn = TRUE)
}
