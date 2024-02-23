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
  } from "svelte-maplibre";
  import { colorByLts, colors, type LayersControls } from "./common";
  import Popup from "./Popup.svelte";
  import PropertiesTable from "./PropertiesTable.svelte";

  export let sourceOverride = {};
  export let controls: LayersControls;

  // TODO The reactivity doesn't seem to see the update to the field
  $: enableControls = !controls.streetviewOn;

  $: lineWidth = getLineWidth(controls.maxCount);
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
</script>

<LineLayer
  id="input-layer"
  {...sourceOverride}
  filter={["==", ["geometry-type"], "LineString"]}
  manageHoverState
  hoverCursor={enableControls ? "pointer" : undefined}
  paint={{
    "line-width": lineWidth,
    "line-color": colorByLts,
    "line-opacity": hoverStateFilter(1.0, 0.5),
  }}
  beforeId="Road labels"
  on:click={openOSM}
>
  {#if enableControls}
    <Popup let:props>
      <PropertiesTable properties={props} />
    </Popup>
  {/if}
</LineLayer>

<CircleLayer
  id="origins-layer"
  {...sourceOverride}
  filter={["has", "origin_count"]}
  manageHoverState
  paint={{
    "circle-color": colors.origins,
    "circle-radius": controls.originRadius,
  }}
  layout={{ visibility: "none" }}
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
  layout={{ visibility: "none" }}
>
  <Popup let:props>
    {props.destination_count} routes end here
  </Popup>
</CircleLayer>
