#!/bin/bash

echo "You probably don't want to run this script, unless you're regenerating all benchmarks. See https://github.com/Urban-Analytics-Technology-Platform/od2net/blob/main/docs/tutorial_examples.md if you're trying to run one example.";
exit 1

function run_example {
	echo "Running example ${1}"
	cd $1

	# Clean up everything from previous runs
	rm -rf input/ intermediate/ output/

	# Create input data
	python3 setup.py

	# Run the pipeline
	cargo run --release -- config.json --output-metadata

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

set -e
set -x

# Small ones
run_example york
run_liverpool_example

# Moderate
run_example edinburgh
run_example london
run_example lisbon

# Huge
run_example england_2011_home_to_work
run_example england_2021_home_to_work
run_example seattle

python3 summarize_results.py */output/metadata.json
