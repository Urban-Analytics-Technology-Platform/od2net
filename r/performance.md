# Performance

|         Example         |Number of routes|Number of edges|Total pipeline time (s)|Routing time (s)|Tippecanoe time (s)|
|-------------------------|----------------|---------------|-----------------------|----------------|-------------------|
|        edinburgh        |      8015      |     56149     |       29.466099       |    2.2114744   |     24.229855     |
|england_2011_home_to_work|    13273135    |    12752209   |        7048.052       |    1751.2865   |     4990.9966     |
|        liverpool        |      3193      |     20866     |       12.830734       |    2.401017    |     7.2867146     |
|          london         |     912015     |     415072    |       226.43355       |    25.040613   |     187.12398     |
|         seattle         |    14485627    |    1502523    |        1181.818       |    513.7812    |     587.61554     |
|           york          |      2500      |      5436     |       3.7256503       |    0.5451243   |     1.2293514     |

- Measurements taken on a...
  - Cores matter (You can tune how many threads the built-in routing uses by setting the `RAYON_NUM_THREADS` environment variable)
  - Preprocessing some areas needs RAM

- Total time breakdown
  - Gathering input (`setup.py`) depends on network speed, and generally steps here aren't optimized for speed
  - Running the pipeline
    - Actually calculating the routes
    - Tippecanoe
    - ... Many other steps, not broken down in the table, and some cached between runs

## Techniques

This pipeline uses a number of techniques to achieve these results on a regular laptop:

- Avoid saving and loading huge intermediate files
  - This is why `ODPattern::LineStrings` is discouraged. Generating requests from patterns is often faster than loading the results of a separate tool.
- Reduce overhead for calling the router
  - The cost of calculating a single route is absolutely tiny. We're calculating millions of routes. So, the overhead for communicating with the router and using the results **must** be tiny.
  - Calling even a local instance of another routing engine over HTTP is very slow. Native bindings should help.
  - There's zero communication overhead with the current in-process approach.
- Use contraction hierarchies, which trade-off upfront time to preprocess the routing graph for much faster queries. Using [fast_paths](https://github.com/easbar/fast_paths/).
- Minimize the results for each routing call
  - Prior approaches have gotten back GeoJSON LineStrings and OSM attributes covering the resulting route. This is incredibly expensive to deal with for many requests.
  - Prior approachs have tried to sum up counts for road segments by [using geometry to represent segments](https://github.com/acteng/overline). This is very slow, has potential floating point errors, can break near bridges/tunnels, etc.
  - Instead, we just ask the router for OSM node IDs (64-bit integers). An edge is just a pair of these. At the very last step when we're generating output GeoJSON to visualize, we can match these node IDs to objects in OSM and produce the same geometry and OSM attributes.
