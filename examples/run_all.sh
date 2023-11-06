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
	rclone copyto output/rnet.pmtiles cloudflare:od2net/output/$1.pmtiles

	cd ..
}

# A special case with two cost functions. Maybe generalize.
function run_liverpool_example {
	echo "Running example liverpool"
	cd liverpool

	# Clean up everything from previous runs
	rm -rf input/ intermediate/ output/

	# Create input data
	python3 setup.py

	# Run the pipeline for the first case and host example output
	cargo run --release -- config_direct.json --output-metadata
	rclone copyto output/rnet.pmtiles cloudflare:od2net/output/liverpool_direct.pmtiles

	# Rerun for the other cost function
	rm -rf intermediate/ output/
	cargo run --release -- config_quiet.json --output-metadata
	rclone copyto output/rnet.pmtiles cloudflare:od2net/output/liverpool_quiet.pmtiles

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
run_liverpool_example

# Moderate
run_example edinburgh
run_example london

# Huge
run_example england_2011_home_to_work
run_example seattle

python3 summarize_results.py */output/metadata.json
