getbbox_from_zones = function() {
  zones = sf::st_read("input/zones.geojson")
  bbox = sf::st_bbox(zones)
  paste0(bbox, collapse = ",")
}

make_osm = function(force_download = FALSE, zones_file = "input/zones.geojson") {
  zones = sf::read_sf(zones_file)
  zones_union = sf::st_union(zones)
  osmextract_match = osmextract::oe_match(place = zones_union)
  osmextract::oe_download(file_url = osmextract_match$url, download_directory = "input", force_download = force_download)
  input_pbf = list.files(path = "input", pattern = basename(osmextract_match$url), full.names = TRUE)
  bb = getbbox_from_zones()
  msg = paste0("osmium extract -b ", bb, " ", input_pbf, " -o input/input.osm.pbf --overwrite")
  system(msg)
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
#' @return NULL
#' @export
make_elevation = function(
    url = NULL,
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
