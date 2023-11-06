#!/bin/bash
# This generates a bunch of osm.pbf files clipped to different cities, for use
# in the interactive web app mode

set -e
set -x

dir=pbf_clips

function clip {
	echo "Creating clip of ${1}"
	curl http://download.geofabrik.de/${2} -o ${dir}/tmp.osm.pbf
	osmium extract -b ${3} ${dir}/tmp.osm.pbf -o ${dir}/${1}.osm.pbf --overwrite
	rm -f ${dir}/tmp.osm.pbf
}

mkdir -p ${dir}

# Use https://download.geofabrik.de/ and http://bboxfinder.com
clip seattle north-america/us/washington-latest.osm.pbf                 -122.365036,47.610561,-122.293968,47.665387
clip london  europe/great-britain/england/greater-london-latest.osm.pbf -0.115528,51.476946,-0.059910,51.505591
clip berlin  europe/germany/berlin-latest.osm.pbf                       13.372421,52.496996,13.447781,52.534289
clip paris   europe/france/ile-de-france-latest.osm.pbf                 2.303867,48.825514,2.387295,48.876571
clip antwerp europe/belgium-latest.osm.pbf                              4.379425,51.185369,4.463196,51.249238

rclone sync pbf_clips/ cloudflare:od2net/pbf_clips/
