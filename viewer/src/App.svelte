<script lang="ts">
  import turfBbox from "@turf/bbox";
  import type { FeatureCollection } from "geojson";
  import type {
    DataDrivenPropertyValueSpecification,
    Map as MapType,
  } from "maplibre-gl";
  import {
    CircleLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapLibre,
    Popup,
  } from "svelte-maplibre";
  import destinationsUrl from "../assets/destination_subpoints.geojson?url";
  import originsUrl from "../assets/origin_subpoints.geojson?url";
  import Histogram from "./Histogram.svelte";
  import Layout from "./Layout.svelte";
  import Legend from "./Legend.svelte";
  import { evaluateLTS } from "./lts";
  import PropertiesTable from "./PropertiesTable.svelte";
  import ToggleLayer from "./ToggleLayer.svelte";

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
  function fileLoaded(e: Event) {
    let reader = new FileReader();
    reader.onload = (e) => {
      loadFile(e.target!.result as string);
    };
    let files = fileInput.files!;
    reader.readAsText(files[0]);
  }

  let map: MapType;

  let gj: FeatureCollection | undefined;
  let endcaps: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };
  let lineWidth: DataDrivenPropertyValueSpecification<number> | undefined;
  let summary: string | undefined;

  let overrideMax = 2000;

  function loadFile(contents: string) {
    let tmp = JSON.parse(contents);
    // Add in LTS
    for (let f of tmp.features) {
      let result = evaluateLTS({ tags: f.properties });
      f.properties.lts = result.lts;
    }

    gj = tmp;
    map.fitBounds(bbox(gj!), { padding: 200, duration: 500 });

    //recalculateEndcaps();

    let min = Number.MAX_VALUE;
    let max = Number.MIN_VALUE;
    for (let f of gj.features) {
      min = Math.min(min, f.properties.count);
      max = Math.max(max, f.properties.count);
    }
    summary = `Counts from ${min} to ${max}`;

    adjustLineWidth(min);
  }

  function recalculateEndcaps() {
    let counts = new Map();
    for (let f of gj.features) {
      for (let pt of [
        f.geometry.coordinates[0],
        f.geometry.coordinates[f.geometry.coordinates.length - 1],
      ]) {
        // TODO Overwrite arbitrarily for now
        counts.set(pt, f.properties.count);
      }
    }
    endcaps.features = [];
    for (let [coordinates, count] of counts.entries()) {
      endcaps.features.push({
        type: "Feature",
        properties: { count },
        geometry: {
          type: "Point",
          coordinates,
        },
      });
    }
    endcaps = endcaps;
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
</script>

<Layout>
  <div slot="left">
    <h1>Latent demand</h1>
    <input bind:this={fileInput} on:change={fileLoaded} type="file" />
    {#if map}
      <ToggleLayer layer="origins-layer" {map}
        ><span style="color: {colors.origins}">Origins</span></ToggleLayer
      >
      <ToggleLayer layer="destinations-layer" {map}
        ><span style="color: {colors.destinations}">Destinations</span
        ></ToggleLayer
      >
    {/if}
    {#if gj}
      <ToggleLayer layer="input-layer" {map}>Route network</ToggleLayer>
      <ToggleLayer layer="endcaps-layer" {map}>Endcaps for routes</ToggleLayer>
      <Legend
        rows={[
          ["LTS 1 - suitable for children", colors.lts1],
          ["LTS 2 - low stress", colors.lts2],
          ["LTS 3 - medium stress", colors.lts3],
          ["LTS 4 - high stress", colors.lts4],
        ]}
      />
      <p>{summary}</p>
      <label>
        Override max for line width styling:
        <input
          type="number"
          bind:value={overrideMax}
          min={1}
          on:change={() => adjustLineWidth(0)}
        />
      </label>
      <Histogram
        title="Edge counts"
        data={gj.features.map((f) => f.properties.count)}
      />
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
      <GeoJSON id="origins" data={originsUrl}>
        <CircleLayer
          id="origins-layer"
          paint={{
            "circle-color": colors.origins,
            "circle-radius": 3,
          }}
          layout={{ visibility: "none" }}
        />
      </GeoJSON>
      <GeoJSON id="destinations" data={destinationsUrl}>
        <CircleLayer
          id="destinations-layer"
          paint={{
            "circle-color": colors.destinations,
            "circle-radius": 10,
          }}
          layout={{ visibility: "none" }}
        />
      </GeoJSON>

      {#if gj}
        <GeoJSON id="endcaps" data={endcaps}>
          <CircleLayer
            id="endcaps-layer"
            paint={{
              "circle-color": "red",
              "circle-radius": ["/", lineWidth, 2.0],
            }}
          />
        </GeoJSON>
        <GeoJSON id="input" data={gj}>
          <LineLayer
            id="input-layer"
            manageHoverState
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
          >
            <Popup openOn="hover" let:features>
              <PropertiesTable properties={features[0].properties} />
            </Popup>
          </LineLayer>
        </GeoJSON>
      {/if}
    </MapLibre>
  </div>
</Layout>