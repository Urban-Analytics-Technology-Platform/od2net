import json
import sys

# Read OSM tags as a JSON dictionary from STDIN
tags = json.loads(sys.stdin.read())

# Output 0 (not allowed), 1 (suitable for children), 2, 3, or 4 (high stress)
if tags["highway"] == "residential":
    print(2)
else:
    print(4)
