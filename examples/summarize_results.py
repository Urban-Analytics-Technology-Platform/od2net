import json
import sys

# This has a dependency on `pip install py-markdown-table`
from markdownTable import markdownTable

rows = []
for arg in sys.argv[1:]:
    with open(arg) as f:
        data = json.load(f)
        rows.append(
            {
                "Example": arg.split("/")[0],
                # TODO Prettyprint counts
                "Number of routes": data["num_requests"],
                "Number of edges": data["num_edges_with_count"],
                "Total pipeline time (s)": data["total_time_seconds"],
                "Routing time (s)": data["routing_time_seconds"],
                "Tippecanoe time (s)": data["tippecanoe_time_seconds"],
            }
        )

print(markdownTable(rows).setParams(row_sep="markdown", quote=False).getMarkdown())
