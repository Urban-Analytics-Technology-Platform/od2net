#!/bin/bash

set -e
set -x

# TODO Check for tools?

function run_example {
	echo "Running example ${1}"
	cd $1

	# Clean up everything from previous runs
	rm -rf input/ intermediate/ output/

	# Create input data
	python3 setup.py

	# Run the pipeline
	cargo run --release -- config.json

	cd ..
}

# Small ones
run_example york
#run_example liverpool	# TODO has some manual steps

# Moderate
run_example edinburgh
run_example london

# Huge
#run_example england_2011_home_to_work
