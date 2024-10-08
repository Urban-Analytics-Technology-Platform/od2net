<script lang="ts">
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
  } from "maplibre-gl";
  import {
    CircleLayer,
    hoverStateFilter,
    LineLayer,
    type LayerClickInfo,
    SymbolLayer,
  } from "svelte-maplibre";
  import {
    slopeLimits,
    slopeColors,
    colorByLts,
    colors,
    type LayersControls,
  } from "./common";
  import { makeColorRamp, Popup } from "svelte-utils/map";
  import {
    showSlope,
    showDestinations,
    showOrigins,
    showRouteNetwork,
  } from "./stores";

  export let sourceOverride = {};
  export let controls: LayersControls;

  // TODO The reactivity doesn't seem to see the update to the field
  $: enableControls = !controls.streetviewOn;

  $: lineWidth = getLineWidth(controls.maxCount);
  function getLineWidth(
    maxCount: number,
  ): DataDrivenPropertyValueSpecification<number> {
    let min = 0;

    // Linearly interpolate between thin and thick, based on the percent each count is between min and max
    let thin = 2;
    let thick = 10;

    let range_input = maxCount - min;
    let range_output = thick - thin;
    // min(1, (value - min) / range_input)
    let calculatePercent: ExpressionSpecification = [
      "min",
      1.0,
      ["/", ["-", ["get", "count"], min], range_input],
    ];
    // thin + range_output * percent
    return ["+", thin, ["*", range_output, calculatePercent]];
  }

  function openOSM(e: CustomEvent<LayerClickInfo>) {
    if (!enableControls) {
      return;
    }
    let id = e.detail.features[0].properties!.way;
    window.open(`http://openstreetmap.org/way/${id}`, "_blank");
  }

  let colorBySlope = makeColorRamp(
    ["abs", ["get", "slope"]],
    slopeLimits,
    slopeColors,
  );
</script>

<LineLayer
  id="input-layer"
  {...sourceOverride}
  filter={["==", ["geometry-type"], "LineString"]}
  manageHoverState
  hoverCursor={enableControls ? "pointer" : undefined}
  paint={{
    "line-width": lineWidth,
    "line-color": $showSlope ? colorBySlope : colorByLts,
    "line-opacity": hoverStateFilter(1.0, 0.5),
  }}
  layout={{
    visibility: $showRouteNetwork ? "visible" : "none",
  }}
  beforeId="Road labels"
  on:click={openOSM}
>
  {#if enableControls}
    <Popup let:props>
      <h2>Count: {props.count.toFixed(1)}</h2>
      <p>
        LTS: <b>{props.lts}</b>
      </p>
      <p>
        Nearby amenities: <b>{props.nearby_amenities}</b>
      </p>
      {#if props.slope}
        <p>
          Slope: <b>{props.slope.toFixed(1)}%</b>
        </p>
      {/if}
      {#if props.forward_cost}
        <p>
          Cost forwards: <b>{props.forward_cost}</b>
        </p>
      {/if}
      {#if props.backward_cost}
        <p>
          Cost backwards: <b>{props.backward_cost}</b>
        </p>
      {/if}
      <p>Click to open OSM</p>
    </Popup>
  {/if}
</LineLayer>

<SymbolLayer
  id="slope-arrows"
  {...sourceOverride}
  filter={[
    "all",
    ["==", ["geometry-type"], "LineString"],
    [">", ["abs", ["get", "slope"]], 3],
  ]}
  minzoom={12}
  layout={{
    "icon-image": "chevron",
    "icon-size": 1.0,
    "symbol-placement": "line",
    "symbol-spacing": 50,
    "icon-allow-overlap": true,
    "icon-rotate": ["case", ["<", ["get", "slope"], 0], 180, 0],
    visibility: $showSlope ? "visible" : "none",
  }}
/>

<CircleLayer
  id="origins-layer"
  {...sourceOverride}
  filter={["has", "origin_count"]}
  manageHoverState
  paint={{
    "circle-color": colors.origins,
    "circle-radius": controls.originRadius,
  }}
  layout={{
    visibility: $showOrigins ? "visible" : "none",
  }}
>
  <Popup let:props>
    {props.origin_count} routes start here
  </Popup>
</CircleLayer>

<CircleLayer
  id="destinations-layer"
  {...sourceOverride}
  filter={["has", "destination_count"]}
  manageHoverState
  paint={{
    "circle-color": colors.destinations,
    "circle-radius": controls.destinationRadius,
  }}
  layout={{
    visibility: $showDestinations ? "visible" : "none",
  }}
>
  <Popup let:props>
    {props.destination_count} routes end here
  </Popup>
</CircleLayer>
