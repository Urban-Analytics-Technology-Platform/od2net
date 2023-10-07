# Performance

| Example |Number of routes|Number of edges|Total pipeline time (s)|Routing time (s)|Tippecanoe time (s)|
|---------|----------------|---------------|-----------------------|----------------|-------------------|
|edinburgh|      8015      |     57735     |         23.64         |       0.2      |       22.17       |
|  london |     901511     |     434202    |         172.23        |      4.56      |       162.45      |
|   york  |      2500      |      6384     |          1.28         |      0.05      |        1.06       |

- Measurements taken on a...
  - Cores matter
  - Preprocessing some areas needs RAM

- Total time breakdown
  - Gathering input (`setup.py`) depends on network speed, and generally steps here aren't optimized for speed
  - Running the pipeline
    - Actually calculating the routes
    - Tippecanoe
    - ... Many other steps, not broken down in the table, and some cached between runs
