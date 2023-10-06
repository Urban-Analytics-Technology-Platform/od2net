import json
import sys
# This has a dependency on `pip install py-markdown-table`
from markdownTable import markdownTable

rows = []
for arg in sys.argv[1:]:
    with open(arg) as f:
        data = json.load(f)
        # TODO Pipeline runtime
        # TODO Just for routing
        # TODO Just for tippecanoe
        # TODO Prettyprint numbers
        rows.append({
            "name": arg.split("/")[0],
            "num_requests": data["num_requests"],
            "num_edges_with_count": data["num_edges_with_count"],
        })

# Print as a Markdown table for convenience
with open("md_table", "w") as f:
    f.write(markdownTable(rows).setParams(row_sep="markdown", quote=False).getMarkdown())
