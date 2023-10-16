<script lang="ts">
  import init from "lts";
  import type { Map as MapType } from "maplibre-gl";
  import { FileAPISource, PMTiles } from "pmtiles";
  import { onMount } from "svelte";
  import { MapLibre } from "svelte-maplibre";
  import { colors } from "./common";
  import Layers from "./Layers.svelte";
  import Layout from "./Layout.svelte";
  import Legend from "./Legend.svelte";
  import Loader from "./Loader.svelte";
  import StreetView from "./StreetView.svelte";
  import ToggleLayer from "./ToggleLayer.svelte";

  onMount(async () => {
    await init();
  });

  let map: MapType;
  let pmtiles: PMTiles | null;
  let example = "";
  let outputMetadata: any | undefined;

  let streetviewOn = false;
  let maxCount = 1000;
  let originRadius = 3;
  let destinationRadius = 3;

  let fileInput: HTMLInputElement;
  function fileLoaded(e: Event) {
    try {
      example = "";
      let files = fileInput.files!;
      pmtiles = new PMTiles(new FileAPISource(files[0]));
    } catch (err) {
      window.alert(
        `Problem loading this PMTiles file. Don't open the GeoJSON file; make sure to select .pmtiles. Error: ${err}`
      );
    }
  }

  $: if (example != "") {
    pmtiles = new PMTiles(
      `http://od2net.s3-website.eu-west-2.amazonaws.com/output/${example}.pmtiles`
    );
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

  function total(meters: number): string {
    let km = meters / 1000.0;
    return `${km.toFixed(1)} km total`;
  }
</script>

<Layout>
  <div slot="left">
    <h1>od2net</h1>
    <label>
      {#if pmtiles == null}
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
          <option value="england_2011_home_to_work"
            >England (2011 home-to-work)</option
          >
          <option value="liverpool">Liverpool</option>
          <option value="london">London</option>
          <option value="seattle">Seattle (Soundcast)</option>
          <option value="seattle_workers">Seattle (workers)</option>
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
    {#if outputMetadata}
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
          [
            `LTS 1 - suitable for children: ${total(
              outputMetadata.total_meters_lts1
            )}`,
            colors.lts1,
          ],
          [
            `LTS 2 - low stress: ${total(outputMetadata.total_meters_lts2)}`,
            colors.lts2,
          ],
          [
            `LTS 3 - medium stress: ${total(outputMetadata.total_meters_lts3)}`,
            colors.lts3,
          ],
          [
            `LTS 4 - high stress: ${total(outputMetadata.total_meters_lts4)}`,
            colors.lts4,
          ],
        ]}
      />
      <p>
        Note: LTS model from <a
          href="https://github.com/BikeOttawa/stressmodel/blob/master/stressmodel.js"
          target="_blank">BikeOttawa</a
        >
      </p>
      <hr />
      <StreetView {map} bind:enabled={streetviewOn} />
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      hash
      bind:map
    >
      {#if outputMetadata}
        {#key outputMetadata}
          <Layers
            {maxCount}
            {originRadius}
            {destinationRadius}
            enableControls={!streetviewOn}
          />
        {/key}
      {/if}
    </MapLibre>
    {#if map}
      <Loader {map} {pmtiles} bind:outputMetadata />
    {/if}
  </div>
</Layout>
