<script lang="ts">
  import { colors, ltsNames } from "./common";
  import Legend from "./Legend.svelte";
  import StreetView from "./StreetView.svelte";
  import ToggleLayer from "./ToggleLayer.svelte";

  export let outputMetadata;
  export let map;
  export let controls;

  function total(meters: number): string {
    let km = meters / 1000.0;
    return `${km.toFixed(1)} km total for all trips (before uptake)`;
  }
</script>

<p>{outputMetadata.config.requests.description}</p>
<div>
  <sp-button
    on:click={() => window.alert(JSON.stringify(outputMetadata, null, "  "))}
    >See all output details</sp-button
  >
</div>
<ToggleLayer layer="input-layer" {map} show>Route network</ToggleLayer>
<div>
  <sp-field-label for="maxCount">Max for line width styling</sp-field-label>
  <sp-number-field
    bind:value={controls.maxCount}
    min={1}
    on:change={() => (controls = controls)}
  />
</div>

<ToggleLayer layer="origins-layer" {map} show={false}
  ><span style="color: {colors.origins}"
    >Origins ({outputMetadata.num_origins.toLocaleString()})</span
  ></ToggleLayer
>
<div>
  <label>
    Change origin point size:<br />
    <input
      type="number"
      bind:value={controls.originRadius}
      min={1}
      on:change={() => (controls = controls)}
    />
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
    <input
      type="number"
      bind:value={controls.destinationRadius}
      min={1}
      on:change={() => (controls = controls)}
    />
  </label>
</div>

<hr />
<Legend
  rows={[
    [
      `${ltsNames.lts1}: ${total(outputMetadata.total_meters_lts1)}`,
      colors.lts1,
    ],
    [
      `${ltsNames.lts2}: ${total(outputMetadata.total_meters_lts2)}`,
      colors.lts2,
    ],
    [
      `${ltsNames.lts3}: ${total(outputMetadata.total_meters_lts3)}`,
      colors.lts3,
    ],
    [
      `${ltsNames.lts4}: ${total(outputMetadata.total_meters_lts4)}`,
      colors.lts4,
    ],
    // Shouldn't happen
    [
      `${ltsNames.lts_not_allowed}: ${total(
        outputMetadata.total_meters_not_allowed
      )}`,
      colors.lts_not_allowed,
    ],
  ]}
/>
<p>
  Note: LTS model from <a
    href="https://github.com/BikeOttawa/stressmodel/blob/master/stressmodel.js"
    target="_blank">BikeOttawa</a
  >
</p>
<hr />
<StreetView {map} bind:enabled={controls.streetviewOn} />
