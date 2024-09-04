<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import chevron from "../assets/chevron.png?url";
  import init from "lts";
  import type { Map as MapType } from "maplibre-gl";
  import { FileSource, PMTiles } from "pmtiles";
  import { onMount } from "svelte";
  import { MapLibre } from "svelte-maplibre";
  import { type LayersControls } from "./common";
  import Header from "./Header.svelte";
  import Layers from "./Layers.svelte";
  import { Layout } from "svelte-utils/top_bar_layout";
  import Loader from "./Loader.svelte";
  import SidebarControls from "./SidebarControls.svelte";

  onMount(async () => {
    await init();
  });

  let map: MapType;
  let pmtiles: PMTiles | null;
  let example = "";
  let outputMetadata: any | undefined;

  let controls: LayersControls = {
    maxCount: 1000,
    originRadius: 3,
    destinationRadius: 3,
    streetviewOn: false,
  };

  // TODO Add Loading screen
  let fileInput: HTMLInputElement;
  function fileLoaded(e: Event) {
    try {
      example = "";
      let files = fileInput.files!;
      pmtiles = new PMTiles(new FileSource(files[0]));
    } catch (err) {
      window.alert(
        `Problem loading this PMTiles file. Don't open the GeoJSON file; make sure to select .pmtiles. Error: ${err}`,
      );
    }
  }

  $: if (example != "") {
    pmtiles = new PMTiles(
      `https://assets.od2net.org/output/${example}.pmtiles`,
    );
  }
</script>

<Layout>
  <div slot="top">
    <Header app="main" />
  </div>
  <div slot="left">
    <details open={!outputMetadata}>
      <summary role="button" class="secondary">Load a file</summary>
      <label>
        {#if pmtiles == null}
          Open a <i>.pmtiles</i>
          file produced by the tool. Note this file stays in your browser; it doesn't
          get uploaded anywhere.
        {/if}
        <input bind:this={fileInput} on:change={fileLoaded} type="file" />
      </label>

      <label>
        Or load an example:
        <select bind:value={example}>
          <option value="">Custom file loaded</option>
          <option value="edinburgh">Edinburgh</option>
          <option value="england_2011_home_to_work">
            England (2011 home-to-work)
          </option>
          <option value="liverpool_direct">Liverpool (direct)</option>
          <option value="liverpool_quiet">Liverpool (quiet)</option>
          <option value="london">London</option>
          <option value="seattle">Seattle</option>
          <option value="york">York</option>
          <option value="lisbon">Lisbon</option>
        </select>
      </label>
    </details>

    {#if outputMetadata}
      <SidebarControls {outputMetadata} {map} bind:controls />
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100%;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      hash
      bind:map
      on:error={(e) => console.log(e.detail)}
      images={[{ id: "chevron", url: chevron }]}
    >
      {#if outputMetadata}
        {#key outputMetadata}
          <Layers
            sourceOverride={{
              source: "pmtilesSource",
              sourceLayer: "rnet",
            }}
            {controls}
          />
        {/key}
      {/if}
    </MapLibre>
    {#if map}
      <Loader {map} {pmtiles} bind:outputMetadata />
    {/if}
  </div>
</Layout>

<style>
  :global(.maplibregl-popup-content) {
    background-color: var(--pico-background-color);
  }

  /* picocss messes up maplibre controls; workaround */
  :global(.maplibregl-ctrl > button) {
    margin-bottom: 0px;
  }
</style>
