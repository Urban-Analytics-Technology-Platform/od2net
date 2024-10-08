<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import { PolygonToolLayer } from "maplibre-draw-polygon";
  import type { FeatureCollection } from "geojson";
  import initLts from "lts";
  import type { Map as MapType } from "maplibre-gl";
  import { onMount } from "svelte";
  import { GeoJSON, MapLibre, Marker } from "svelte-maplibre";
  import init, { JsNetwork } from "wasm-od2net";
  import markerSvg from "../assets/marker.svg?raw";
  import ClippedPBFs from "./ClippedPBFs.svelte";
  import { type Cost, type LayersControls } from "./common";
  import CostFunction from "./CostFunction.svelte";
  import Header from "./Header.svelte";
  import Layers from "./Layers.svelte";
  import { Layout } from "svelte-utils/top_bar_layout";
  import { Loading } from "svelte-utils";
  import { OverpassSelector } from "svelte-utils/overpass";
  import SidebarControls from "./SidebarControls.svelte";

  onMount(async () => {
    await init();
    await initLts();
  });

  let map: MapType;
  let network: JsNetwork | undefined;
  let example = "";
  let markerPosition = { lng: 0.0, lat: 0.0 };
  let gj: FeatureCollection & { metadata?: any } = {
    type: "FeatureCollection",
    features: [],
  };
  let loading = "";

  let maxRequests = 1000;
  let cost: Cost = "Distance";
  let controls: LayersControls = {
    maxCount: 1000,
    originRadius: 3,
    destinationRadius: 3,
    streetviewOn: false,
  };

  let fileInput: HTMLInputElement;
  async function fileLoaded(e: Event) {
    example = "";
    loading = "Loading file";
    loadBytes(await fileInput.files![0].arrayBuffer());
  }

  function loadBytes(buffer: ArrayBuffer) {
    try {
      network = new JsNetwork(new Uint8Array(buffer));
      cost = "Distance";

      let bbox = network.getBounds();
      map.fitBounds(
        [
          [bbox[0], bbox[1]],
          [bbox[2], bbox[3]],
        ],
        { padding: 20, animate: false },
      );
      markerPosition.lng = (bbox[0] + bbox[2]) / 2.0;
      markerPosition.lat = (bbox[1] + bbox[3]) / 2.0;
      recalculate();
    } catch (err) {
      window.alert(`Problem importing osm.pbf file: ${err}`);
    }
    loading = "";
  }

  async function loadExample(example: string) {
    if (example != "") {
      loading = `Loading ${example}`;
      let resp = await fetch(
        `https://assets.od2net.org/pbf_clips/${example}.osm.pbf`,
      );
      loadBytes(await resp.arrayBuffer());
    }
  }

  $: loadExample(example);

  function recalculate() {
    if (!network) {
      return;
    }
    gj = JSON.parse(
      network.recalculate({
        lng: markerPosition.lng,
        lat: markerPosition.lat,
        max_requests: maxRequests,
        cost,
      }),
    );
  }

  function onUpdate(cost: Cost, maxRequests: number) {
    recalculate();
  }
  $: onUpdate(cost, maxRequests);

  function gotXml(e: CustomEvent<string>) {
    loading = "Parsing XML";
    // TODO Can we avoid turning into bytes?
    loadBytes(new TextEncoder().encode(e.detail));
    loading = "";
  }
</script>

<Layout>
  <div slot="top">
    <Header app="interactive" />
  </div>
  <div slot="left">
    <details open={!network}>
      <summary role="button" class="secondary">Change areas</summary>
      <label>
        Open an <i>.osm.pbf</i>
        file
        <input bind:this={fileInput} on:change={fileLoaded} type="file" />
      </label>
      <ClippedPBFs bind:example />
      <OverpassSelector
        {map}
        on:gotXml={gotXml}
        on:loading={(e) => (loading = e.detail)}
        on:error={(e) => (loading = e.detail)}
      />
    </details>

    {#if network}
      <label>
        Max requests (limit for faster updates):
        <br />
        <input type="number" bind:value={maxRequests} min={1} />
      </label>
      <CostFunction bind:cost />
    {/if}

    {#if gj.metadata}
      <hr />
      <SidebarControls outputMetadata={gj.metadata} {map} bind:controls />
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      hash
      bind:map
    >
      <Marker bind:lngLat={markerPosition} draggable on:dragend={recalculate}>
        {@html markerSvg}
      </Marker>
      <GeoJSON data={gj}>
        <Layers {controls} />
      </GeoJSON>
      <PolygonToolLayer />
    </MapLibre>
  </div>
</Layout>

<Loading {loading} />

<style>
  :global(.maplibregl-popup-content) {
    background-color: var(--pico-background-color);
  }

  /* picocss messes up maplibre controls; workaround */
  :global(.maplibregl-ctrl > button) {
    margin-bottom: 0px;
  }
</style>
