import json
import sys

# Output a numeric edge cost
def calculate(edge):
    tags = edge["osm_tags"]
    length_meters = edge["length_meters"]
    lts = edge["lts"]
    nearby_amenities = edge["nearby_amenities"]
    slope = edge["slope"]

    # Return None to not use the edge at all

    if tags["highway"] == "residential":
        return [round(length_meters), round(length_meters)]
    else:
        # Strongly avoid non-residential roads
        return [round(10 * length_meters), round(10 * length_meters)]


# Read an array of JSON dictionaries from STDIN
input_batch = json.loads(sys.stdin.read())
# Calculate an edge cost for each one
results = list(map(calculate, input_batch))
# Write a JSON array of the resulting numbers
print(json.dumps(results))
