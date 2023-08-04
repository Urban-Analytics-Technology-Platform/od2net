<script lang="ts">
  import turfBbox from "@turf/bbox";
  import type { FeatureCollection } from "geojson";
  import type { DataDrivenPropertyValueSpecification, Map } from "maplibre-gl";
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapLibre,
    Popup,
  } from "svelte-maplibre";
  import Histogram from "./Histogram.svelte";
  import Layout from "./Layout.svelte";
  import PropertiesTable from "./PropertiesTable.svelte";

  let fileInput: HTMLInputElement;
  function fileLoaded(e: Event) {
    let reader = new FileReader();
    reader.onload = (e) => {
      loadFile(e.target!.result as string);
    };
    let files = fileInput.files!;
    reader.readAsText(files[0]);
  }

  let map: Map;

  let gj: FeatureCollection | undefined;
  let lineWidth: DataDrivenPropertyValueSpecification<number> | undefined;
  let summary: string | undefined;
  function loadFile(contents: string) {
    gj = JSON.parse(contents);
    map.fitBounds(bbox(gj!), { padding: 200, duration: 500 });

    let min = Number.MAX_VALUE;
    let max = Number.MIN_VALUE;
    for (let f of gj.features) {
      min = Math.min(min, f.properties.count);
      max = Math.max(max, f.properties.count);
    }
    summary = `Counts from ${min} to ${max}`;

    // Linearly interpolate between thin and thick, based on the percent each count is between min and max
    let thin = 2;
    let thick = 10;

    let range_input = max - min;
    let range_output = thick - thin;
    // thin + range_output * (value - min) / range_input
    lineWidth = [
      "+",
      thin,
      ["/", ["*", range_output, ["-", ["get", "count"], min]], range_input],
    ];
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
    {#if summary}
      <p>{summary}</p>{/if}
    {#if gj}
      <Histogram
        title="Edge counts"
        data={gj.features.map((f) => f.properties.count)}
      />
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/streets/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      bind:map
    >
      {#if gj}
        <GeoJSON id="input" data={gj}>
          <LineLayer
            manageHoverState
            paint={{
              "line-width": lineWidth,
              "line-color": "red",
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
