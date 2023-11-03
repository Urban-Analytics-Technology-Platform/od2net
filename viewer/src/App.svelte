<script lang="ts">
  import init from "lts";
  import type { Map as MapType } from "maplibre-gl";
  import { FileAPISource, PMTiles } from "pmtiles";
  import { onMount } from "svelte";
  import { MapLibre } from "svelte-maplibre";
  import Header from "./Header.svelte";
  import Layers from "./Layers.svelte";
  import Layout from "./Layout.svelte";
  import Loader from "./Loader.svelte";
  import SidebarControls from "./SidebarControls.svelte";

  onMount(async () => {
    await init();
  });

  let map: MapType;
  let pmtiles: PMTiles | null;
  let example = "";
  let outputMetadata: any | undefined;

  let controls = {
    maxCount: 1000,
    originRadius: 3,
    destinationRadius: 3,
    streetviewOn: false,
  };

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
</script>

<Layout>
  <div slot="left">
    <Header app="main" />
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
          <option value="seattle">Seattle</option>
          <option value="york">York</option>
        </select>
      </label>
    </div>
    {#if outputMetadata}
      <SidebarControls {outputMetadata} {map} {controls} />
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
