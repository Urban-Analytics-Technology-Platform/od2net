

The setup information is contained within the `setup.py` file, which
generates minimal input files.

``` {bash}
python setup.py
```

We can visualise these as follows:

``` python
import geopandas as gpd
import pandas as pd
zones = gpd.read_file("input/zones.geojson")
destinations = gpd.read_file("input/destinations.geojson")
od = pd.read_csv("input/od.csv")
ax = zones.plot()
destinations.plot(ax=ax, color='red')
```

![](README_files/figure-commonmark/cell-2-output-1.png)
