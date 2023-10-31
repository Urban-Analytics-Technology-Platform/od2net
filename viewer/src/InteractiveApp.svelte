<script lang="ts">
  import initLts from "lts";
  import type { Map as MapType } from "maplibre-gl";
  import { onMount } from "svelte";
  import { GeoJSON, MapLibre, Marker } from "svelte-maplibre";
  import init, { JsNetwork } from "wasm-od2net";
  import Layers from "./Layers.svelte";
  import Layout from "./Layout.svelte";
  import SidebarControls from "./SidebarControls.svelte";

  onMount(async () => {
    await init();
    await initLts();
  });

  let map: MapType;
  let network: JsNetwork | undefined;
  let markerPosition = { lat: 53.937, lng: -1.0159 };
  let gj = {
    type: "FeatureCollection",
    features: [],
  };

  let maxRequests = 1000;
  let controls = {
    maxCount: 1000,
    originRadius: 3,
    destinationRadius: 3,
    streetviewOn: false,
  };

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
    gj = JSON.parse(
      network.recalculate({
        lng: markerPosition.lng,
        lat: markerPosition.lat,
        max_requests: maxRequests,
      })
    );
    window.gj = gj;
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
      <div>
        <label>
          Max requests (limit for faster updates):<br />
          <input type="number" bind:value={maxRequests} min={1} />
        </label>
      </div>
    {/if}

    {#if gj.metadata}
      <hr />
      <SidebarControls outputMetadata={gj.metadata} {map} {controls} />
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      hash
      bind:map
    >
      <Marker bind:lngLat={markerPosition} draggable
        ><p style="background: red">X</p></Marker
      >
      <GeoJSON data={gj}>
        <Layers {controls} />
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>
