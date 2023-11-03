<script lang="ts">
  import initLts from "lts";
  import type { Map as MapType } from "maplibre-gl";
  import { onMount } from "svelte";
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapLibre,
    Popup,
  } from "svelte-maplibre";
  import init, { JsNetwork } from "wasm-od2net";
  import { colors } from "./common";
  import Layout from "./Layout.svelte";
  import PropertiesTable from "./PropertiesTable.svelte";

  onMount(async () => {
    await init();
    await initLts();
  });

  let map: MapType;
  let network: JsNetwork | undefined;
  let gj = {
    type: "FeatureCollection",
    features: [],
  };

  let fileInput: HTMLInputElement;
  async function fileLoaded(e: Event) {
    try {
      let buffer = await fileInput.files![0].arrayBuffer();
      network = new JsNetwork(new Uint8Array(buffer));

      let bbox = network.getBounds();
      map.fitBounds(
        [
          [bbox[0], bbox[1]],
          [bbox[2], bbox[3]],
        ],
        { padding: 20, animate: false }
      );
      gj = JSON.parse(network.debugNetwork());
    } catch (err) {
      window.alert(`Problem loading network file: ${err}`);
    }
  }

  function openOSM(feature) {
    let id = feature.properties.way;
    window.open(`http://openstreetmap.org/way/${id}`, "_blank");
  }
</script>

<Layout>
  <div slot="left">
    <h1>od2net edge cost explorer</h1>
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
    >
      <GeoJSON data={gj}>
        <LineLayer
          manageHoverState
          hoverCursor="pointer"
          paint={{
            "line-width": 3.0,
            // TODO Bucket whatever thing we're looking at
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
          beforeId="Road labels"
          on:click={(e) => openOSM(e.detail.features[0])}
        >
          <Popup openOn="hover" let:features>
            <PropertiesTable properties={features[0].properties} />
          </Popup>
        </LineLayer>
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>
