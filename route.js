import * as fs from "fs";
import fetch from "node-fetch";

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

async function main() {
  let urls = generateRequestUrls();

  for (let url of urls) {
    let nodes = await nodesForRequest(url);
    console.log(`Got ${nodes.length} nodes`);
  }
}
