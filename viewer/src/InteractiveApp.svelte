<script lang="ts">
  import type { Map as MapType } from "maplibre-gl";
  import { onMount } from "svelte";
  import { MapLibre } from "svelte-maplibre";
  import init, { JsNetwork } from "wasm-od2net";
  import Layout from "./Layout.svelte";

  onMount(async () => {
    await init();
  });

  let map: MapType;

  let fileInput: HTMLInputElement;
  async function fileLoaded(e: Event) {
    try {
      let buffer = await fileInput.files![0].arrayBuffer();
      let network = new JsNetwork(new Uint8Array(buffer));
      console.log(`everything worked! ${network.tmp()}`);
    } catch (err) {
      window.alert(`Problem loading network file: ${err}`);
    }
  }
</script>

<Layout>
  <div slot="left">
    <h1>od2net interactive mode</h1>
    <label>
      Open a <i>.bin</i> network file
      <input bind:this={fileInput} on:change={fileLoaded} type="file" />
    </label>
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      hash
      bind:map
    />
  </div>
</Layout>
