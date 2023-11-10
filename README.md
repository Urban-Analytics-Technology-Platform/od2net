# od2net

![Logo](viewer/assets/logo.png)

WARNING: This repo is not ready for general use. The API and input/output formats will keep changing. There are big limitations not yet documented.

od2net helps you turn *o*rigin/*d*estination data about where people travel into a cycling *net*work plan. You can use this to decide what streets to prioritize for safety improvements.

1.  You specify origins and destinations of trips that cyclists take today or of short trips taken by car
2.  You specify how cyclists would like to choose routes. Are fast/direct routes important, or sticking to quiet streets? Do you want to route near greenspace and shops?)
3.  od2net calculates routes very quickly, counting how many routes cross every street
4.  You use the web viewer to look at the most popular streets, filtering for streets that don't have good cycling infrastructure today

## How to use it

...

- set up with your own data, run on your computer (directly or with docker), and compute country-wide network with millions of trips in under an hour
- the quick setup route: clip a small area from OSM, use dummy OD data, tune cost function, and make route networks ending at a single point. interactive in your browser, no install required, get something in minutes

## Contributing

We'd love contributions of all sorts -- developers, designers, data scientists, and applying it somewhere new! Check out [GitHub Issues](https://github.com/Urban-Analytics-Technology-Platform/od2net/issues), file a new one, or email <dabreegster@gmail.com> to get started.

This project follows the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) and is Apache 2.0 licensed. See all [credits](docs/credits.md).







