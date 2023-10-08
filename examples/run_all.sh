#!/bin/bash

function run_example {
	echo "Running example ${1}"
	cd $1

	# Clean up everything from previous runs
	rm -rf input/ intermediate/ output/

	# Create input data
	python3 setup.py

	# Run the pipeline
	cargo run --release -- config.json --output-metadata
	# TODO or with docker

	# Host example output
	#aws s3 cp output/rnet.pmtiles s3://od2net/output/$1.pmtiles

	cd ..
}

function check_dependencies {
	echo "Checking dependencies"

	# TODO Slightly different for docker
	for dep in python3 cargo curl ogr2ogr tippecanoe osmium gunzip; do
		if which $dep > /dev/null; then
			true
		else
			echo "You're missing a dependency: $dep";
			exit 1;
		fi
	done
}

check_dependencies

set -e
set -x

# Small ones
run_example york
#run_example liverpool	# TODO something's broken with setup

# Moderate
run_example edinburgh
run_example london

# Huge
#run_example england_2011_home_to_work
#run_example seattle

python3 summarize_results.py */output/metadata.json
