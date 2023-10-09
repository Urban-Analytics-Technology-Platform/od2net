<script lang="ts">
  import init from "lts";
  import maplibregl from "maplibre-gl";
  import type { Map as MapType } from "maplibre-gl";
  import { FileAPISource, PMTiles, Protocol } from "pmtiles";
  import { onMount } from "svelte";
  import { MapLibre, VectorTileSource } from "svelte-maplibre";
  import { colors } from "./common";
  import Layers from "./Layers.svelte";
  import Layout from "./Layout.svelte";
  import Legend from "./Legend.svelte";
  import ToggleLayer from "./ToggleLayer.svelte";

  onMount(async () => {
    await init();
  });

  function cleanupSource(id: string) {
    if (map.getSource(id)) {
      // First remove all layers using this source
      let layers = [];
      for (let layer of map.getStyle().layers) {
        if ("source" in layer && layer.source == id) {
          layers.push(layer.id);
        }
      }
      for (let layer of layers) {
        map.removeLayer(layer);
      }

      map.removeSource(id);
    }
  }

  let fileInput: HTMLInputElement;
  async function fileLoaded(e: Event) {
    try {
      let files = fileInput.files!;
      let pmtilesFile = new PMTiles(new FileAPISource(files[0]));
      let protocol = new Protocol();
      maplibregl.addProtocol("pmtiles", protocol.tile);
      protocol.add(pmtilesFile);

      let header = await pmtilesFile.getHeader();
      let bounds: [number, number, number, number] = [
        header.minLon,
        header.minLat,
        header.maxLon,
        header.maxLat,
      ];

      let source = "pmtilesSource";
      // Teardown previous file if needed
      // TODO Do we need to removeProtocol? Any memory leak?
      cleanupSource(source);

      map.addSource(source, {
        type: "vector",
        tiles: ["pmtiles://" + pmtilesFile.source.getKey() + "/{z}/{x}/{y}"],
        minzoom: header.minZoom,
        maxzoom: header.maxZoom,
        bounds,
      });
      map.fitBounds(bounds, { padding: 100, duration: 500 });

      let metadata = await pmtilesFile.getMetadata();
      outputMetadata = JSON.parse(metadata.description);

      loadedFileCount++;
      example = "";
    } catch (err) {
      window.alert(
        `Problem loading this PMTiles file. Don't open the GeoJSON file; make sure to select .pmtiles. Error: ${err}`
      );
    }
  }

  let map: MapType;

  // TODO Use a counter to recreate layers after cleaning up a source. Hack.
  let loadedFileCount = 0;
  let example = "";
  let outputMetadata: any | undefined;

  let maxCount = 1000;
  let originRadius = 3;
  let destinationRadius = 3;

  $: if (example == "") {
    // TODO Hack...
    loadedFileCount = 0;
  }

  $: if (map) {
    map.on("moveend", () => {
      let counts = [];
      // TODO To be paranoid, dedupe by feature ID;
      for (let f of map.queryRenderedFeatures(undefined, {
        layers: ["input-layer"],
      })) {
        counts.push(f.properties.count);
      }
      let min = Math.min(...counts);
      let max = Math.max(...counts);
      let count = counts.length;
      // TODO Displaying a histogram could be helpful, at least to debug
      console.log({ min, max, count });
    });
  }
</script>

<Layout>
  <div slot="left">
    <h1>od2net</h1>
    <label>
      {#if loadedFileCount == 0}
        Open a <i>.pmtiles</i> file produced by the tool. Note this file stays in
        your browser; it doesn't get uploaded anywhere.
      {/if}
      <input bind:this={fileInput} on:change={fileLoaded} type="file" />
    </label>
    <div>
      <label>
        Or load an example:
        <select bind:value={example}>
          <option value="">Custom file loaded</option>
          <option value="edinburgh">Edinburgh</option>
          <option value="london">London</option>
          <option value="york">York</option>
        </select>
      </label>
    </div>
    {#if outputMetadata}
      <p>{outputMetadata.config.requests.description}</p>
      <div>
        <button
          on:click={() =>
            window.alert(JSON.stringify(outputMetadata, null, "  "))}
          >See all output details</button
        >
      </div>
    {/if}
    {#if loadedFileCount > 0}
      <ToggleLayer layer="input-layer" {map} show>Route network</ToggleLayer>
      <div>
        <label>
          Max for line width styling:<br />
          <input type="number" bind:value={maxCount} min={1} />
        </label>
      </div>

      <ToggleLayer layer="origins-layer" {map} show={false}
        ><span style="color: {colors.origins}"
          >Origins ({outputMetadata.num_origins.toLocaleString()})</span
        ></ToggleLayer
      >
      <div>
        <label>
          Change origin point size:<br />
          <input type="number" bind:value={originRadius} min={1} />
        </label>
      </div>

      <ToggleLayer layer="destinations-layer" {map} show={false}
        ><span style="color: {colors.destinations}"
          >Destinations ({outputMetadata.num_destinations.toLocaleString()})</span
        ></ToggleLayer
      >
      <div>
        <label>
          Change destination point size:<br />
          <input type="number" bind:value={destinationRadius} min={1} />
        </label>
      </div>

      <hr />
      <Legend
        rows={[
          ["LTS 1 - suitable for children", colors.lts1],
          ["LTS 2 - low stress", colors.lts2],
          ["LTS 3 - medium stress", colors.lts3],
          ["LTS 4 - high stress", colors.lts4],
        ]}
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
      hash
      bind:map
    >
      {#if loadedFileCount > 0}
        {#key loadedFileCount}
          <Layers {maxCount} {originRadius} {destinationRadius} />
        {/key}
      {/if}
      {#if example != ""}
        <VectorTileSource
          id="pmtilesSource"
          url={`pmtiles://http://od2net.s3-website.eu-west-2.amazonaws.com/output/${example}.pmtiles`}
        >
          <Layers {maxCount} {originRadius} {destinationRadius} />
        </VectorTileSource>
      {/if}
    </MapLibre>
  </div>
</Layout>
