<script lang="ts">
  import init from "lts";
  import maplibregl from "maplibre-gl";
  import type {
    DataDrivenPropertyValueSpecification,
    Map as MapType,
  } from "maplibre-gl";
  import { FileAPISource, PMTiles, Protocol } from "pmtiles";
  import { onMount } from "svelte";
  import {
    CircleLayer,
    hoverStateFilter,
    LineLayer,
    MapLibre,
    Popup,
  } from "svelte-maplibre";
  import Layout from "./Layout.svelte";
  import Legend from "./Legend.svelte";
  import PropertiesTable from "./PropertiesTable.svelte";
  import ToggleLayer from "./ToggleLayer.svelte";

  onMount(async () => {
    await init();
  });

  let colors = {
    origins: "blue",
    destinations: "purple",

    lts1: "#009e73",
    lts2: "#56b4e9",
    lts3: "#e69f00",
    lts4: "#d55e00",
    lts_unknown: "black",
  };

  function cleanupSource(id: string) {
    if (map.getSource(id)) {
      // First remove all layers using this source
      let layers = [];
      for (let layer of map.getStyle().layers) {
        if ("source" in layer && layer.source == id) {
          layers.push(layer.id);
        }
      }
      for (let layer of layers) {
        map.removeLayer(layer);
      }

      map.removeSource(id);
    }
  }

  let fileInput: HTMLInputElement;
  async function fileLoaded(e: Event) {
    try {
      let files = fileInput.files!;
      let pmtilesFile = new PMTiles(new FileAPISource(files[0]));
      let protocol = new Protocol();
      maplibregl.addProtocol("pmtiles", protocol.tile);
      protocol.add(pmtilesFile);

      let header = await pmtilesFile.getHeader();
      let bounds: [number, number, number, number] = [
        header.minLon,
        header.minLat,
        header.maxLon,
        header.maxLat,
      ];

      let source = "pmtilesSource";
      // Teardown previous file if needed
      // TODO Do we need to removeProtocol? Any memory leak?
      cleanupSource(source);

      map.addSource(source, {
        type: "vector",
        tiles: ["pmtiles://" + pmtilesFile.source.getKey() + "/{z}/{x}/{y}"],
        minzoom: header.minZoom,
        maxzoom: header.maxZoom,
        bounds,
      });
      map.fitBounds(bounds, { padding: 100, duration: 500 });
      adjustLineWidth();

      let metadata = await pmtilesFile.getMetadata();
      outputMetadata = JSON.parse(metadata.description);

      loadedFileCount++;
    } catch (err) {
      window.alert(
        `Problem loading this PMTiles file. Don't open the GeoJSON file; make sure to select .pmtiles. Error: ${err}`
      );
    }
  }

  let map: MapType;

  // TODO Use a counter to recreate layers after cleaning up a source. Hack.
  let loadedFileCount = 0;
  let lineWidth: DataDrivenPropertyValueSpecification<number> | undefined;
  let outputMetadata: any | undefined;

  let max = 1000;
  let originRadius = 3;
  let destinationRadius = 3;

  function adjustLineWidth() {
    let min = 0;

    // Linearly interpolate between thin and thick, based on the percent each count is between min and max
    let thin = 2;
    let thick = 10;

    let range_input = max - min;
    let range_output = thick - thin;
    // min(1, (value - min) / range_input)
    let calculatePercent = [
      "min",
      1.0,
      ["/", ["-", ["get", "count"], min], range_input],
    ];
    // thin + range_output * percent
    lineWidth = ["+", thin, ["*", range_output, calculatePercent]];
  }

  function openOSM(feature) {
    let id = feature.properties.way;
    window.open(`http://openstreetmap.org/way/${id}`, "_blank");
  }
</script>

<Layout>
  <div slot="left">
    <h1>Latent demand</h1>
    <label>
      {#if loadedFileCount == 0}
        Open a <i>.pmtiles</i> file produced by the tool. Note this file stays in
        your browser; it doesn't get uploaded anywhere.
      {/if}
      <input bind:this={fileInput} on:change={fileLoaded} type="file" />
    </label>
    {#if outputMetadata}
      <div>
        <button
          on:click={() =>
            window.alert(JSON.stringify(outputMetadata.config, null, "  "))}
          >See config</button
        >
      </div>
    {/if}
    {#if loadedFileCount > 0}
      <ToggleLayer layer="input-layer" {map} show>Route network</ToggleLayer>
      <div>
        <label>
          Max for line width styling:<br />
          <input
            type="number"
            bind:value={max}
            min={1}
            on:change={() => adjustLineWidth()}
          />
        </label>
      </div>

      <ToggleLayer layer="origins-layer" {map} show={false}
        ><span style="color: {colors.origins}"
          >Origins ({outputMetadata.num_origins.toLocaleString()})</span
        ></ToggleLayer
      >
      <div>
        <label>
          Change origin point size:<br />
          <input type="number" bind:value={originRadius} min={1} />
        </label>
      </div>

      <ToggleLayer layer="destinations-layer" {map} show={false}
        ><span style="color: {colors.destinations}"
          >Destinations ({outputMetadata.num_destinations.toLocaleString()})</span
        ></ToggleLayer
      >
      <div>
        <label>
          Change destination point size:<br />
          <input type="number" bind:value={destinationRadius} min={1} />
        </label>
      </div>

      <hr />
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
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      bind:map
    >
      {#if loadedFileCount > 0}
        {#key loadedFileCount}
          <LineLayer
            id="input-layer"
            source="pmtilesSource"
            sourceLayer="rnet"
            filter={["==", ["geometry-type"], "LineString"]}
            manageHoverState
            hoverCursor="pointer"
            paint={{
              "line-width": lineWidth,
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

          <CircleLayer
            id="origins-layer"
            source="pmtilesSource"
            sourceLayer="rnet"
            filter={["has", "origin_count"]}
            manageHoverState
            paint={{
              "circle-color": colors.origins,
              "circle-radius": originRadius,
            }}
            layout={{ visibility: "none" }}
          >
            <Popup openOn="hover" let:features>
              {features[0].properties.origin_count} routes start here
            </Popup>
          </CircleLayer>

          <CircleLayer
            id="destinations-layer"
            source="pmtilesSource"
            sourceLayer="rnet"
            filter={["has", "destination_count"]}
            manageHoverState
            paint={{
              "circle-color": colors.destinations,
              "circle-radius": destinationRadius,
            }}
            layout={{ visibility: "none" }}
          >
            <Popup openOn="hover" let:features>
              {features[0].properties.destination_count} routes end here
            </Popup>
          </CircleLayer>
        {/key}
      {/if}
    </MapLibre>
  </div>
</Layout>
