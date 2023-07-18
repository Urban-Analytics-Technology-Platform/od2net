import * as fs from "fs";
import fetch from "node-fetch";
import { SingleBar } from "cli-progress";
import { createOSMStream } from 'osm-pbf-parser-node';

main();

async function main() {
  let countPerEdge = await calculateRoutes();
  await generateRouteNetwork(countPerEdge);
}

// Returns countPerEdge
async function calculateRoutes() {
  // Maps from two OSM node IDs to a count of routes crossing
  let countPerEdge = new Map();

  let urls = generateRequestUrls();
  console.log(`Calculating ${urls.length} routes`);
  console.time(`Calculating routes`);
  let requests = urls.map(url => nodesForRequest(url));

  // TODO This requires everything to succeed
  let responses = await Promise.all(requests);
  for (let nodes of responses) {
    for (let i = 0; i < nodes.length - 1; i++) {
      let key = `${nodes[i]},${nodes[i + 1]}`;
      let count = countPerEdge.get(key) ?? 0;
      countPerEdge.set(key, count + 1);
    }
  }
  console.timeEnd(`Calculating routes`);

  return countPerEdge;
}

function generateRequestUrls() {
  let requests = JSON.parse(fs.readFileSync("input/requests.geojson"));
  let urls = [];
  for (let request of requests.features) {
    let coords = request.geometry.coordinates;
    let x1 = coords[0][0];
    let y1 = coords[0][1];
    let x2 = coords[1][0];
    let y2 = coords[1][1];
    let url = `http://localhost:5000/route/v1/driving/${x1},${y1};${x2},${y2}?overview=false&alternatives=false&steps=false&annotations=nodes`;
    urls.push(url);
  }
  return urls;
}

// Returns a list of OSM node IDs, or throws
async function nodesForRequest(url) {
  let resp = await fetch(url);
  let json = await resp.json();
  if (json.code != "Ok") {
    throw new Error(`${url} failed: ${JSON.stringify(json)}`);
  }
  return json.routes[0].legs[0].annotation.nodes;
}

async function generateRouteNetwork(countPerEdge) {
  // Create the node lookup if needed, or load it
  let nodes;
  try {
    console.log(`Loading node lookup table`);
    console.time(`Loading node lookup table`);
    nodes = JSON.parse(fs.readFileSync("nodes.json"));
    console.timeEnd(`Loading node lookup table`);
  } catch (err) {
    console.log(`Node lookup table not there, building it...`);
    nodes = await buildNodeLookupTable("osrm/london.osm.pbf");
    console.log(`Saving node lookup table for next time...`);
    fs.writeFileSync("nodes.json", JSON.stringify(nodes));
  }

  console.log(`Turning route network into geometry`);
  let gj = {
    type: "FeatureCollection",
    features: []
  };
  for (let [key, count] of countPerEdge) {
    let [node1, node2] = key;
    let pos1 = nodes[node1];
    let pos2 = nodes[node2];
    gj.features.push({
      type: "Feature",
      geometry: {
        type: "LineString",
        coordinates: [pos1, pos2],
      },
      properties: {
        count,
        node1: `https://www.openstreetmap.org/node/${node1}`,
        node2: `https://www.openstreetmap.org/node/${node2}`,
      }
    });
  }
  fs.writeFileSync("output.geojson", JSON.stringify(gj));
}

async function buildNodeLookupTable(path) {
  let nodes = {};
  let count = 0;
  for await (let item of createOSMStream(path, { withTags: false })) {
    if (item.type == "node") {
      nodes[item.id] = [item.lon, item.lat];
      count++;
      if (count % 100000 == 0) {
        console.log(`Scraped ${count} nodes`);
      }
    }
  }
  return nodes;
}
