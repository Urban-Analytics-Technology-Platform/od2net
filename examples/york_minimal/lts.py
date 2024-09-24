import json
import sys

# Output 0 (not allowed), 1 (suitable for children), 2 (low stress), 3 (low stress), or 4 (high stress)
def calculate(tags):
    if tags["highway"] == "residential":
        return 2
    else:
        return 4


# Read an array of JSON dictionaries from STDIN
tags_batch = json.loads(sys.stdin.read())
# Calculate LTS for each one
lts_results = list(map(calculate, tags_batch))
# Write a JSON array of the resulting numbers
print(json.dumps(lts_results))
