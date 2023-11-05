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
  import { colorByLts, colors, colorScale, makeColorRamp } from "./common";
  import Header from "./Header.svelte";
  import Layout from "./Layout.svelte";
  import Legend from "./Legend.svelte";
  import PropertiesTable from "./PropertiesTable.svelte";
  import SequentialLegend from "./SequentialLegend.svelte";

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
  let colorBy: "lts" | "cost" | "nearby_amenities" = "lts";

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

  // TODO Could just be fixed objects, not functions
  function lineColorBy(colorBy) {
    if (colorBy == "lts") {
      return colorByLts;
    } else if (colorBy == "cost") {
      return makeColorRamp(
        ["/", ["get", "cost"], ["get", "length"]],
        limitsFor(colorBy),
        colorScale
      );
    } else if (colorBy == "nearby_amenities") {
      return makeColorRamp(
        ["get", "nearby_amenities"],
        limitsFor(colorBy),
        colorScale
      );
    }
  }

  function limitsFor(colorBy) {
    if (colorBy == "lts") {
      return null;
    } else if (colorBy == "cost") {
      return equalBins(1.0, 30.0);
    } else if (colorBy == "nearby_amenities") {
      return equalBins(0, 20);
    }
  }

  function equalBins(min, max) {
    let result = [];
    let step = (max - min) / 5.0;
    for (let i = 0; i < 6; i++) {
      result.push(min + i * step);
    }
    return result;
  }
</script>

<Layout>
  <div slot="left">
    <Header app="costs" />
    <label>
      Open a <i>.bin</i> network file or an <i>.osm.pbf</i>
      <input bind:this={fileInput} on:change={fileLoaded} type="file" />
    </label>
    <select bind:value={colorBy}>
      <option value="lts">LTS</option>
      <option value="cost">Edge cost (relative to length)</option>
      <option value="nearby_amenities">Nearby amenities</option>
    </select>
    {#if colorBy == "lts"}
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
    {:else}
      <SequentialLegend {colorScale} limits={limitsFor(colorBy)} />
    {/if}
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
            "line-width": 5.0,
            "line-color": lineColorBy(colorBy),
            "line-opacity":
              colorBy == "nearby_amenities"
                ? [
                    "case",
                    ["==", 0, ["get", "nearby_amenities"]],
                    0.0,
                    hoverStateFilter(1.0, 0.5),
                  ]
                : hoverStateFilter(1.0, 0.5),
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
