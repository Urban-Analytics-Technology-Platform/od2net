# od2net

![Logo](web/assets/logo.png)

WARNING: This repo is not ready for general use. The API and input/output formats will keep changing. There are big limitations not yet documented.

od2net helps you turn *o*rigin/*d*estination data about where people travel into a cycling *net*work plan. You can use this to decide what streets to prioritize for safety improvements.

1.  You specify origins and destinations of trips that cyclists take today or of short trips taken by car
2.  You specify how cyclists would like to choose routes. Are fast/direct routes important, or sticking to quiet streets? Do you want to route near greenspace and shops?)
3.  od2net calculates routes very quickly, counting how many routes cross every street
4.  You use the web viewer to look at the most popular streets, filtering for streets that don't have good cycling infrastructure today

## How to use it

You can quickly try out od2net without installing anything:

- [Exploring pre-built examples](https://od2net.org)
- [Generating a route network in your browser for a small area](http://od2net.org/interactive.html)

Once you're ready to run in a large area with your own origin/destination data, then start with the [tutorial](docs/tutorial_examples.md) and see [all documentation](https://github.com/Urban-Analytics-Technology-Platform/od2net/tree/main/docs). Again note this project status is still pre-alpha; docs are not all written yet.

### Installation

```shell
cargo install --locked --git https://github.com/Urban-Analytics-Technology-Platform/od2net
```

### Docker

You can run the application with Docker, e.g. as follows:

```shell
sudo docker run -v $(pwd):/app ghcr.io/urban-analytics-technology-platform/od2net:main /app/config.json
```

### With Python

See Python scripts in the [examples](examples) directory, as described in the [tutorial](docs/tutorial_examples.md).

### R package

There is a small [R package](https://github.com/Urban-Analytics-Technology-Platform/od2net/tree/main/r) with functions to help generate the input files needed by od2net. See documentation [here](https://urban-analytics-technology-platform.github.io/od2net/r/).

## Contributing

We'd love contributions of all sorts -- developers, designers, data scientists, and applying it somewhere new! Check out [GitHub Issues](https://github.com/Urban-Analytics-Technology-Platform/od2net/issues), file a new one, or email <dabreegster@gmail.com> to get started.

This project follows the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) and is Apache 2.0 licensed. See all [credits](docs/credits.md).
