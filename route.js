import * as fs from "fs";
import fetch from "node-fetch";
import { SingleBar } from "cli-progress";

main();

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

// Uses Overpass to get the coordinates of an OSM node.
async function osmNodeCoordinates(node) {
  let url = `https://overpass-api.de/api/interpreter?data=[out:json]; node(${node}); out;`;
  let resp = await fetch(url);
  let json = await resp.json();
  return [json.elements[0].lon, json.elements[0].lat];
}

async function main() {
  let urls = generateRequestUrls();

  // Maps from two OSM node IDs to a count of routes crossing. Stringifies keys, because JS.
  let countPerEdge = {};

  let progress = new SingleBar();
  console.log(`Calculating routes`);
  progress.start(urls.length, 0);
  for (let url of urls) {
    progress.increment();
    let nodes = await nodesForRequest(url);
    for (let i = 0; i < nodes.length - 1; i++) {
      let key = `${nodes[i]},${nodes[i + 1]}`;
      countPerEdge[key] ||= 0;
      countPerEdge[key]++;
    }
  }
  progress.stop();

  // Turn the most common segments into GJ (slowly, relying on Overpass)
  let commonEdges = Object.entries(countPerEdge).sort((a, b) => b[1] - a[1]).slice(0, 10);
  let gj = {
    type: "FeatureCollection",
    features: []
  };
  console.log(`Turning route network into geometry`);
  progress.start(commonEdges.length, 0);
  for (let [key, count] of commonEdges) {
    progress.increment();
    let [node1, node2] = key.split(",");
    let pos1 = await osmNodeCoordinates(node1);
    let pos2 = await osmNodeCoordinates(node2);
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
  progress.stop();
  fs.writeFileSync("output.geojson", JSON.stringify(gj));
}
