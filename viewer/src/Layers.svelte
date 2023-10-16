<script lang="ts">
  import type { DataDrivenPropertyValueSpecification } from "maplibre-gl";
  import {
    CircleLayer,
    hoverStateFilter,
    LineLayer,
    Popup,
  } from "svelte-maplibre";
  import { colors } from "./common";
  import PropertiesTable from "./PropertiesTable.svelte";

  export let maxCount: number;
  export let originRadius: number;
  export let destinationRadius: number;
  export let enableControls: boolean;

  $: lineWidth = getLineWidth(maxCount);
  function getLineWidth(
    maxCount: number
  ): DataDrivenPropertyValueSpecification<number> {
    let min = 0;

    // Linearly interpolate between thin and thick, based on the percent each count is between min and max
    let thin = 2;
    let thick = 10;

    let range_input = maxCount - min;
    let range_output = thick - thin;
    // min(1, (value - min) / range_input)
    let calculatePercent = [
      "min",
      1.0,
      ["/", ["-", ["get", "count"], min], range_input],
    ];
    // thin + range_output * percent
    return ["+", thin, ["*", range_output, calculatePercent]];
  }

  function openOSM(feature) {
    if (!enableControls) {
      return;
    }
    let id = feature.properties.way;
    window.open(`http://openstreetmap.org/way/${id}`, "_blank");
  }
</script>

<LineLayer
  id="input-layer"
  source="pmtilesSource"
  sourceLayer="rnet"
  filter={["==", ["geometry-type"], "LineString"]}
  manageHoverState
  hoverCursor={enableControls ? "pointer" : undefined}
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
  {#if enableControls}
    <Popup openOn="hover" let:features>
      <PropertiesTable properties={features[0].properties} />
    </Popup>
  {/if}
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
