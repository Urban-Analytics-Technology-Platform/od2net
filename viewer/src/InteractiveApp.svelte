<script lang="ts">
  import type { Map as MapType } from "maplibre-gl";
  import { onMount } from "svelte";
  import { Marker, MapLibre } from "svelte-maplibre";
  import init, { JsNetwork } from "wasm-od2net";
  import Layout from "./Layout.svelte";

  onMount(async () => {
    await init();
  });

  let map: MapType;
  let network: JsNetwork | undefined;
  let markerPosition = { lng: -10, lat: -20 };

  let fileInput: HTMLInputElement;
  async function fileLoaded(e: Event) {
    try {
      let buffer = await fileInput.files![0].arrayBuffer();
      network = new JsNetwork(new Uint8Array(buffer));
    } catch (err) {
      window.alert(`Problem loading network file: ${err}`);
    }
  }

  function recalculate() {
    network.recalculate({
      lng: markerPosition.lng,
      lat: markerPosition.lat,
    });
  }
</script>

<Layout>
  <div slot="left">
    <h1>od2net interactive mode</h1>
    <label>
      Open a <i>.bin</i> network file
      <input bind:this={fileInput} on:change={fileLoaded} type="file" />
    </label>

    {#if network}
      <button on:click={recalculate}>Recalculate</button>
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      hash
      bind:map
    >
       <Marker bind:lngLat={markerPosition} draggable><p style="background: red">X</p></Marker>
    </MapLibre>
  </div>
</Layout>
