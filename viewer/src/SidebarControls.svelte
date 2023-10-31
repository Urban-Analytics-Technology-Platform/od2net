<script lang="ts">
  import { colors } from "./common";
  import Legend from "./Legend.svelte";
  import StreetView from "./StreetView.svelte";
  import ToggleLayer from "./ToggleLayer.svelte";

  export let outputMetadata;
  export let map;
  export let controls;

  function total(meters: number): string {
    let km = meters / 1000.0;
    return `${km.toFixed(1)} km total`;
  }
</script>

<p>{outputMetadata.config.requests.description}</p>
<div>
  <button
    on:click={() => window.alert(JSON.stringify(outputMetadata, null, "  "))}
    >See all output details</button
  >
</div>
<ToggleLayer layer="input-layer" {map} show>Route network</ToggleLayer>
<div>
  <label>
    Max for line width styling:<br />
    <input type="number" bind:value={controls.maxCount} min={1} />
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
    <input type="number" bind:value={controls.originRadius} min={1} />
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
    <input type="number" bind:value={controls.destinationRadius} min={1} />
  </label>
</div>

<hr />
<Legend
  rows={[
    [
      `LTS 1 - suitable for children: ${total(
        outputMetadata.total_meters_lts1
      )}`,
      colors.lts1,
    ],
    [
      `LTS 2 - low stress: ${total(outputMetadata.total_meters_lts2)}`,
      colors.lts2,
    ],
    [
      `LTS 3 - medium stress: ${total(outputMetadata.total_meters_lts3)}`,
      colors.lts3,
    ],
    [
      `LTS 4 - high stress: ${total(outputMetadata.total_meters_lts4)}`,
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
<hr />
<StreetView {map} bind:enabled={controls.streetviewOn} />
