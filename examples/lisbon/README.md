This example builds a route network for Lisbon with elevation data.

To get it running you can use either setup.py or setup.R.

For the R implemenation run the following commands in the R console from the inputs/lisbon directory:

```r
# Generates the function definitions in the R environment:
source("setup.R")
main()
system("cargo run --release config.json")
```

Then open a browser, e.g. with:

```r
browseURL("https://od2net.org")
```
