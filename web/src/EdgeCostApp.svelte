<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import { PolygonToolLayer } from "maplibre-draw-polygon";
  import type { FeatureCollection } from "geojson";
  import initLts from "lts";
  import type {
    Map as MapType,
    DataDrivenPropertyValueSpecification,
  } from "maplibre-gl";
  import { onMount } from "svelte";
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapLibre,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import init, { JsNetwork } from "wasm-od2net";
  import ClippedPBFs from "./ClippedPBFs.svelte";
  import {
    colorByLts,
    colors,
    colorScale,
    ltsNames,
    type Cost,
  } from "./common";
  import CostFunction from "./CostFunction.svelte";
  import Header from "./Header.svelte";
  import { Layout } from "svelte-utils/top_bar_layout";
  import { Popup, makeColorRamp } from "svelte-utils/map";
  import {
    Loading,
    SequentialLegend,
    Legend,
    PropertiesTable,
  } from "svelte-utils";
  import { OverpassSelector } from "svelte-utils/overpass";

  onMount(async () => {
    await init();
    await initLts();
  });

  type ColorBy = "lts" | "cost" | "nearby_amenities";

  let map: MapType;
  let network: JsNetwork | undefined;
  let example = "";
  let gj: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };
  let cost: Cost = "Distance";
  let colorBy: ColorBy = "cost";
  let showNotAllowed = false;
  let loading = "";
  // Note the 0th entry is "not allowed"; it won't be filled out at all
  let percentByLength = [0, 0, 0, 0, 0];
  let maxCostRatio = 1.0;
  let maxNearbyAmenities = 1;

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
      updateGj();
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

  function updateGj() {
    gj = JSON.parse(network!.debugNetwork());
    let allSum = 0;
    let ltsSum = [0, 0, 0, 0, 0];
    maxCostRatio = 0.0;
    maxNearbyAmenities = 0;
    for (let f of gj.features) {
      let props = f.properties!;
      maxNearbyAmenities = Math.max(maxNearbyAmenities, props.nearby_amenities);

      // A "not allowed" edge without a cost or length
      if (!props.length) {
        continue;
      }
      maxCostRatio = Math.max(maxCostRatio, props.forward_cost / props.length);

      allSum += props.length;
      ltsSum[props.lts] += props.length;
    }
    percentByLength = ltsSum.map((x) => (x / allSum) * 100);
  }

  function openOSM(e: CustomEvent<LayerClickInfo>) {
    let id = e.detail.features[0].properties!.way;
    window.open(`http://openstreetmap.org/way/${id}`, "_blank");
  }

  function lineColorBy(
    colorBy: ColorBy,
    maxCostRatio: number,
    maxNearbyAmenities: number,
  ): DataDrivenPropertyValueSpecification<string> {
    if (colorBy == "lts") {
      return colorByLts;
    } else if (colorBy == "cost") {
      return [
        "case",
        ["==", 0, ["get", "lts"]],
        colors.lts_not_allowed,
        // @ts-expect-error TODO Not sure the problem
        makeColorRamp(
          ["/", ["get", "forward_cost"], ["get", "length"]],
          limitsFor(colorBy, maxCostRatio, maxNearbyAmenities),
          colorScale,
        ),
      ];
    } else if (colorBy == "nearby_amenities") {
      return makeColorRamp(
        ["get", "nearby_amenities"],
        limitsFor(colorBy, maxCostRatio, maxNearbyAmenities),
        colorScale,
      );
    } else {
      throw new Error("unreachable");
    }
  }

  function lineOpacity(
    colorBy: ColorBy,
    showNotAllowed: boolean,
  ): DataDrivenPropertyValueSpecification<number> {
    let hover = hoverStateFilter(1.0, 0.5);
    if (colorBy == "nearby_amenities") {
      return ["case", ["==", 0, ["get", "nearby_amenities"]], 0.0, hover];
    }
    if (showNotAllowed) {
      return hover;
    }
    return ["case", ["==", 0, ["get", "lts"]], 0.0, hover];
  }

  function limitsFor(
    colorBy: ColorBy,
    maxCostRatio: number,
    maxNearbyAmenities: number,
  ): number[] {
    if (colorBy == "lts") {
      return [];
    } else if (colorBy == "cost") {
      return equalBins(0.0, maxCostRatio);
    } else if (colorBy == "nearby_amenities") {
      return equalBins(0, maxNearbyAmenities);
    } else {
      throw new Error("unreachable");
    }
  }

  function equalBins(min: number, max: number): number[] {
    let result = [];
    let step = (max - min) / 5.0;
    for (let i = 0; i < 6; i++) {
      result.push(min + i * step);
    }
    return result;
  }

  function gotXml(e: CustomEvent<string>) {
    loading = "Parsing XML";
    // TODO Can we avoid turning into bytes?
    loadBytes(new TextEncoder().encode(e.detail));
    loading = "";
  }

  function updateCost(cost: Cost) {
    if (network) {
      network.updateCostFunction(cost);
      updateGj();
    }
  }
  $: updateCost(cost);
</script>

<Layout>
  <div slot="top">
    <Header app="costs" />
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
        Color edges by:
        <select bind:value={colorBy}>
          <option value="lts">LTS</option>
          <option value="cost">Edge cost (relative to length)</option>
          <option value="nearby_amenities">Nearby amenities</option>
        </select>
      </label>

      {#if colorBy == "lts"}
        <Legend
          rows={[
            [
              `${ltsNames.lts1}: ${percentByLength[1].toFixed(
                0,
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
            target="_blank"
          >
            BikeOttawa
          </a>
        </p>
      {:else}
        <SequentialLegend
          {colorScale}
          limits={limitsFor(colorBy, maxCostRatio, maxNearbyAmenities)}
          decimalPlaces={1}
        />
      {/if}

      <label style:color={colors.lts_not_allowed}>
        <input type="checkbox" bind:checked={showNotAllowed} />
        Show cyclists not allowed
      </label>

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
              maxNearbyAmenities,
            ),
            "line-opacity": lineOpacity(colorBy, showNotAllowed),
          }}
          beforeId="Road labels"
          on:click={openOSM}
        >
          <Popup let:props>
            <div style="max-width: 30vw; max-height: 60vh; overflow: auto;">
              <PropertiesTable properties={props} />
            </div>
          </Popup>
        </LineLayer>
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
