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
  import ClippedPBFs from "./ClippedPBFs.svelte";
  import {
    colorByLts,
    colors,
    colorScale,
    ltsNames,
    makeColorRamp,
  } from "./common";
  import CostFunction from "./CostFunction.svelte";
  import Header from "./Header.svelte";
  import Layout from "./Layout.svelte";
  import Legend from "./Legend.svelte";
  import Loading from "./Loading.svelte";
  import OverpassSelector from "./OverpassSelector.svelte";
  import PropertiesTable from "./PropertiesTable.svelte";
  import SequentialLegend from "./SequentialLegend.svelte";

  onMount(async () => {
    await init();
    await initLts();
  });

  let map: MapType;
  let network: JsNetwork | undefined;
  let example = "";
  let gj = {
    type: "FeatureCollection",
    features: [],
  };
  let cost = "Distance";
  let colorBy: "lts" | "cost" | "nearby_amenities" = "cost";
  let showNotAllowed = false;
  let loading = false;
  // Note the 0th entry is "not allowed"; it won't be filled out at all
  let percentByLength = [0, 0, 0, 0, 0];
  let maxCostRatio = 1.0;
  let maxNearbyAmenities = 1;

  let fileInput: HTMLInputElement;
  async function fileLoaded(e: Event) {
    example = "";
    loading = true;
    loadBytes(await fileInput.files![0].arrayBuffer());
  }

  function loadBytes(buffer) {
    try {
      network = new JsNetwork(new Uint8Array(buffer));
      cost = "Distance";

      let bbox = network.getBounds();
      map.fitBounds(
        [
          [bbox[0], bbox[1]],
          [bbox[2], bbox[3]],
        ],
        { padding: 20, animate: false }
      );
      updateGj();
    } catch (err) {
      window.alert(`Problem importing osm.pbf file: ${err}`);
    }
    loading = false;
  }

  async function loadExample(example) {
    if (example != "") {
      loading = true;
      let resp = await fetch(
        `https://assets.od2net.org/pbf_clips/${example}.osm.pbf`
      );
      loadBytes(await resp.arrayBuffer());
    }
  }

  $: loadExample(example);

  function updateGj() {
    gj = JSON.parse(network.debugNetwork());
    let allSum = 0;
    let ltsSum = [0, 0, 0, 0, 0];
    maxCostRatio = 0.0;
    maxNearbyAmenities = 0;
    for (let f of gj.features) {
      maxNearbyAmenities = Math.max(
        maxNearbyAmenities,
        f.properties.nearby_amenities
      );

      // A "not allowed" edge without a cost or length
      if (!f.properties.length) {
        continue;
      }
      maxCostRatio = Math.max(
        maxCostRatio,
        f.properties.cost / f.properties.length
      );

      allSum += f.properties.length;
      ltsSum[f.properties.lts] += f.properties.length;
    }
    percentByLength = ltsSum.map((x) => (x / allSum) * 100);
  }

  function openOSM(feature) {
    let id = feature.properties.way;
    window.open(`http://openstreetmap.org/way/${id}`, "_blank");
  }

  function lineColorBy(colorBy, maxCostRatio, maxNearbyAmenities) {
    if (colorBy == "lts") {
      return colorByLts;
    } else if (colorBy == "cost") {
      return [
        "case",
        ["==", 0, ["get", "lts"]],
        colors.lts_not_allowed,
        makeColorRamp(
          ["/", ["get", "cost"], ["get", "length"]],
          limitsFor(colorBy, maxCostRatio, maxNearbyAmenities),
          colorScale
        ),
      ];
    } else if (colorBy == "nearby_amenities") {
      return makeColorRamp(
        ["get", "nearby_amenities"],
        limitsFor(colorBy, maxCostRatio, maxNearbyAmenities),
        colorScale
      );
    }
  }

  function lineOpacity(colorBy, showNotAllowed) {
    let hover = hoverStateFilter(1.0, 0.5);
    if (colorBy == "nearby_amenities") {
      return ["case", ["==", 0, ["get", "nearby_amenities"]], 0.0, hover];
    }
    if (showNotAllowed) {
      return hover;
    }
    return ["case", ["==", 0, ["get", "lts"]], 0.0, hover];
  }

  function limitsFor(colorBy, maxCostRatio, maxNearbyAmenities) {
    if (colorBy == "lts") {
      return null;
    } else if (colorBy == "cost") {
      return equalBins(0.0, maxCostRatio);
    } else if (colorBy == "nearby_amenities") {
      return equalBins(0, maxNearbyAmenities);
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

  let overpassMessage = "";
  function gotXml(e: CustomEvent<string>) {
    overpassMessage = "Parsing XML";
    // TODO Can we avoid turning into bytes?
    loadBytes(new TextEncoder().encode(e.detail));
    overpassMessage = "";
  }

  function updateCost(cost) {
    if (network) {
      network.updateCostFunction(cost);
      updateGj();
    }
  }
  $: updateCost(cost);
</script>

<Layout>
  <div slot="left">
    <Header app="costs" />
    <label>
      Open an <i>.osm.pbf</i> file
      <input bind:this={fileInput} on:change={fileLoaded} type="file" />
    </label>
    <ClippedPBFs bind:example />
    <OverpassSelector
      {map}
      on:gotXml={gotXml}
      on:loading={(e) => (overpassMessage = e.detail)}
      on:error={(e) => (overpassMessage = e.detail)}
    />
    {#if overpassMessage}
      <p>{overpassMessage}</p>
    {/if}

    {#if network}
      <hr />
      <div>
        <label>
          Color edges by:
          <select bind:value={colorBy}>
            <option value="lts">LTS</option>
            <option value="cost">Edge cost (relative to length)</option>
            <option value="nearby_amenities">Nearby amenities</option>
          </select>
        </label>
      </div>
      {#if colorBy == "lts"}
        <Legend
          rows={[
            [
              `${ltsNames.lts1}: ${percentByLength[1].toFixed(
                0
              )}% of roads by distance`,
              colors.lts1,
            ],
            [
              `${ltsNames.lts2}: ${percentByLength[2].toFixed(0)}%`,
              colors.lts2,
            ],
            [
              `${ltsNames.lts3}: ${percentByLength[3].toFixed(0)}%`,
              colors.lts3,
            ],
            [
              `${ltsNames.lts4}: ${percentByLength[4].toFixed(0)}%`,
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
      {:else}
        <SequentialLegend
          {colorScale}
          limits={limitsFor(colorBy, maxCostRatio, maxNearbyAmenities)}
        />
      {/if}
      <div>
        <label style:color={colors.lts_not_allowed}>
          <input type="checkbox" bind:checked={showNotAllowed} />
          Show cyclists not allowed
        </label>
      </div>
      <hr />
      <CostFunction bind:cost />
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
            "line-color": lineColorBy(
              colorBy,
              maxCostRatio,
              maxNearbyAmenities
            ),
            "line-opacity": lineOpacity(colorBy, showNotAllowed),
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
<Loading {loading} />
