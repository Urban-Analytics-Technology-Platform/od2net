<script lang="ts">
  import turfBbox from "@turf/bbox";
  import type { Feature, FeatureCollection } from "geojson";
  import init from "lts";
  import maplibregl from "maplibre-gl";
  import type {
    DataDrivenPropertyValueSpecification,
    Map as MapType,
  } from "maplibre-gl";
  import { FileAPISource, PMTiles, Protocol } from "pmtiles";
  import { onMount } from "svelte";
  import {
    CircleLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapLibre,
    Popup,
  } from "svelte-maplibre";
  import Layout from "./Layout.svelte";
  import Legend from "./Legend.svelte";
  import { evaluateLTS } from "./lts";
  import PropertiesTable from "./PropertiesTable.svelte";
  import ToggleLayer from "./ToggleLayer.svelte";

  onMount(async () => {
    await init();
  });

  let colors = {
    origins: "blue",
    destinations: "purple",

    lts1: "#009e73",
    lts2: "#56b4e9",
    lts3: "#e69f00",
    lts4: "#d55e00",
    lts_unknown: "black",
  };

  let fileInput: HTMLInputElement;
  async function fileLoaded(e: Event) {
    let files = fileInput.files!;
    let pmtilesFile = new PMTiles(new FileAPISource(files[0]));
    let protocol = new Protocol();
    maplibregl.addProtocol("pmtiles", protocol.tile);
    protocol.add(pmtilesFile);

    let header = await pmtilesFile.getHeader();
    let bounds = [header.minLon, header.minLat, header.maxLon, header.maxLat];
    map.addSource("pmtilesSource", {
      type: "vector",
      tiles: ["pmtiles://" + pmtilesFile.source.getKey() + "/{z}/{x}/{y}"],
      minzoom: header.minZoom,
      maxzoom: header.maxZoom,
      bounds,
    });
    map.fitBounds(bounds, { padding: 100, duration: 500 });
    adjustLineWidth(1);

    let metadata = await pmtilesFile.getMetadata();
    config = JSON.parse(metadata.description);

    gotPmtiles = true;
  }

  let map: MapType;

  // TODO Rename as 'loaded' or something, or maybe even store the protocol to later unload it
  let gotPmtiles = false;
  let rnetGj: FeatureCollection | undefined;
  let lineWidth: DataDrivenPropertyValueSpecification<number> | undefined;
  let summary: string | undefined;
  let config: any | undefined;

  let overrideMax = 1000;
  let originRadius = 3;
  let destinationRadius = 3;

  function loadFile(contents: string) {
    rnetGj = {
      type: "FeatureCollection",
      features: [],
    };

    let gj = JSON.parse(contents);
    config = gj.config;

    let min = Number.MAX_VALUE;
    let max = Number.MIN_VALUE;
    for (let f of gj.features) {
      if (f.geometry.type == "LineString") {
        min = Math.min(min, f.properties.count);
        max = Math.max(max, f.properties.count);

        // TODO The Rust LTS isn't ready yet
        let result = evaluateLTS({ tags: f.properties.osm_tags });
        f.properties.lts = result.lts;

        rnetGj.features.push(f);
      }
    }

    map.fitBounds(bbox(rnetGj!), { padding: 100, duration: 500 });

    summary = `Route segment counts from ${min.toFixed(2)} to ${max.toFixed(
      2
    )}`;

    adjustLineWidth(min);

    // Make sure Svelte sees the update
    rnetGj = rnetGj;
  }

  function adjustLineWidth(min: number) {
    // Manually fake the max, and clamp below
    let max = overrideMax;

    // Linearly interpolate between thin and thick, based on the percent each count is between min and max
    let thin = 2;
    let thick = 10;

    let range_input = max - min;
    let range_output = thick - thin;
    // min(1, (value - min) / range_input)
    let calculatePercent = [
      "min",
      1.0,
      ["/", ["-", ["get", "count"], min], range_input],
    ];
    // thin + range_output * percent
    lineWidth = ["+", thin, ["*", range_output, calculatePercent]];
  }

  // Suitable for passing to map.fitBounds. Work around https://github.com/Turfjs/turf/issues/1807.
  function bbox(gj: FeatureCollection): [number, number, number, number] {
    return turfBbox(gj) as [number, number, number, number];
  }

  function openOSM(feature: Feature) {
    let id = feature.properties.way;
    window.open(`http://openstreetmap.org/way/${id}`, "_blank");
  }
</script>

<Layout>
  <div slot="left">
    <h1>Latent demand</h1>
    <input bind:this={fileInput} on:change={fileLoaded} type="file" />
    {#if config}
      <div>
        <button
          on:click={() => window.alert(JSON.stringify(config, null, "  "))}
          >See config</button
        >
      </div>
    {/if}
    {#if gotPmtiles}
      <ToggleLayer layer="input-layer" {map} show>Route network</ToggleLayer>
      <ToggleLayer layer="origins-layer" {map} show={false}
        ><span style="color: {colors.origins}">Origins</span></ToggleLayer
      >
      <ToggleLayer layer="destinations-layer" {map} show={false}
        ><span style="color: {colors.destinations}">Destinations</span
        ></ToggleLayer
      >
      <Legend
        rows={[
          ["LTS 1 - suitable for children", colors.lts1],
          ["LTS 2 - low stress", colors.lts2],
          ["LTS 3 - medium stress", colors.lts3],
          ["LTS 4 - high stress", colors.lts4],
        ]}
      />
      <p>{summary}</p>

      <div>
        <label>
          Override max for line width styling:<br />
          <input
            type="number"
            bind:value={overrideMax}
            min={1}
            on:change={() => adjustLineWidth(0)}
          />
        </label>
      </div>
      <div>
        <label>
          Change origin point size:<br />
          <input type="number" bind:value={originRadius} min={1} />
        </label>
      </div>
      <div>
        <label>
          Change destination point size:<br />
          <input type="number" bind:value={destinationRadius} min={1} />
        </label>
      </div>

      <p>
        Note: LTS model from <a
          href="https://github.com/BikeOttawa/stressmodel/blob/master/stressmodel.js"
          target="_blank">BikeOttawa</a
        >
      </p>
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      bind:map
    >
      {#if gotPmtiles}
        <LineLayer
          id="input-layer"
          source="pmtilesSource"
          sourceLayer="rnet"
          filter={["==", ["geometry-type"], "LineString"]}
          manageHoverState
          hoverCursor="pointer"
          paint={{
            "line-width": lineWidth,
            "line-color": [
              // Colors from https://github.com/BikeOttawa/maps.bikeottawa.ca-frontend/blob/master/lts/index.html
              "match",
              ["get", "lts"],
              1,
              colors.lts1,
              2,
              colors.lts2,
              3,
              colors.lts3,
              4,
              colors.lts4,
              colors.lts_unknown,
            ],
            "line-opacity": hoverStateFilter(1.0, 0.5),
          }}
          beforeId="Road labels"
          on:click={(e) => openOSM(e.detail.features[0])}
        >
          <Popup openOn="hover" let:features>
            <PropertiesTable properties={features[0].properties} />
          </Popup>
        </LineLayer>

        <CircleLayer
          id="origins-layer"
          source="pmtilesSource"
          sourceLayer="rnet"
          filter={["has", "origin_count"]}
          manageHoverState
          paint={{
            "circle-color": colors.origins,
            "circle-radius": originRadius,
          }}
          layout={{ visibility: "none" }}
        >
          <Popup openOn="hover" let:features>
            {features[0].properties.origin_count} routes start here
          </Popup>
        </CircleLayer>

        <CircleLayer
          id="destinations-layer"
          source="pmtilesSource"
          sourceLayer="rnet"
          filter={["has", "destination_count"]}
          manageHoverState
          paint={{
            "circle-color": colors.destinations,
            "circle-radius": destinationRadius,
          }}
          layout={{ visibility: "none" }}
        >
          <Popup openOn="hover" let:features>
            {features[0].properties.destination_count} routes end here
          </Popup>
        </CircleLayer>
      {/if}
    </MapLibre>
  </div>
</Layout>
