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
#' @param output_file The file path or name of the output OSM file in PBF format. Default is "input/input.osm.pbf".
#'
#' @return This function does not return any value. It downloads and extracts OSM data based on the specified zones.
#'
#' @examples
#' if (file.exists("input/zones.geojson")) {
#'   make_osm(force_download = TRUE, zones_file = "input/zones.geojson")
#' }
#' @export
make_osm = function(
  force_download = FALSE,
  zones_file = "input/zones.geojson",
  output_file = "input/input.osm.pbf"
  ) {
  zones = sf::read_sf(zones_file)
  zones_union = sf::st_union(zones)
  osmextract_match = osmextract::oe_match(place = zones_union)
  osmextract::oe_download(file_url = osmextract_match$url, download_directory = "input", force_download = force_download)
  input_pbf = list.files(path = "input", pattern = basename(osmextract_match$url), full.names = TRUE)
  bb = getbbox_from_zones()
  msg = paste0("osmium extract -b ", bb, " ", input_pbf, " -o ", output_file, " --overwrite")
  system(msg)
}

#' make_origins function
#'
#' This function reads an OpenStreetMap file, selects the multipolygons with a non-null building attribute,
#' calculates the centroids of the selected buildings, and writes the resulting centroids to a GeoJSON file.
#' 
#' @param osm_file The file path or name of the OSM file in PBF format. Default is "input/input.osm.pbf".
#' @param query The SQL query to select the multipolygons with a non-null building attribute. Default is "SELECT osm_id FROM multipolygons WHERE building IS NOT NULL".
#' @param output_file The file path or name of the output GeoJSON file containing the centroids of the selected buildings. Default is "input/buildings.geojson".
#'
#' @return None
#' @export
make_origins = function(
  osm_file = "input/input.osm.pbf",
  query = "SELECT osm_id FROM multipolygons WHERE building IS NOT NULL",
  output_file = "input/buildings.geojson"
  ) {
  buildings = sf::read_sf(osm_file, query = query)
  use_sf = sf::sf_use_s2(FALSE)
  centroids = sf::st_centroid(buildings)
  sf::sf_use_s2(use_sf)
  sf::write_sf(centroids, output_file, delete_dsn = TRUE)
}
