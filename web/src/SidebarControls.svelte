<script lang="ts">
  import type { Map } from "maplibre-gl";
  import {
    slopeLimits,
    slopeColors,
    colors,
    ltsNames,
    type LayersControls,
  } from "./common";
  import { SequentialLegend, Legend } from "svelte-utils";
  import {
    showSlope,
    showDestinations,
    showOrigins,
    showRouteNetwork,
  } from "./stores";
  import StreetView from "./StreetView.svelte";

  export let outputMetadata: any;
  export let map: Map;
  export let controls: LayersControls;

  function total(meters: number): string {
    let km = meters / 1000.0;
    return `${km.toFixed(1)} km total for all trips (before uptake)`;
  }
</script>

<p>{outputMetadata.config.requests.description}</p>
<button
  on:click={() => window.alert(JSON.stringify(outputMetadata, null, "  "))}
>
  See all output details
</button>

<details>
  <summary role="button" class="secondary">Route network layer</summary>

  <label>
    <input type="checkbox" bind:checked={$showRouteNetwork} />
    Route network
  </label>

  <label>
    Max for line width styling:
    <br />
    <input
      type="number"
      bind:value={controls.maxCount}
      min={1}
      on:change={() => (controls = controls)}
    />
  </label>

  <label>
    <input type="checkbox" bind:checked={$showOrigins} />
    <span style="color: {colors.origins}">
      Origins ({outputMetadata.num_origins.toLocaleString()})
    </span>
  </label>

  <label>
    Change origin point size:
    <br />
    <input
      type="number"
      bind:value={controls.originRadius}
      min={1}
      on:change={() => (controls = controls)}
    />
  </label>

  <label>
    <input type="checkbox" bind:checked={$showDestinations} />
    <span style="color: {colors.destinations}">
      Destinations ({outputMetadata.num_destinations.toLocaleString()})
    </span>
  </label>

  <label>
    Change destination point size:
    <br />
    <input
      type="number"
      bind:value={controls.destinationRadius}
      min={1}
      on:change={() => (controls = controls)}
    />
  </label>

  {#if outputMetadata.config.elevation_geotiff}
    <label>
      <input type="checkbox" bind:checked={$showSlope} />
      Visualize slope
    </label>
  {/if}

  {#if $showSlope}
    <SequentialLegend colorScale={slopeColors} limits={slopeLimits} />
  {:else}
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
            outputMetadata.total_meters_not_allowed,
          )}`,
          colors.lts_not_allowed,
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
  {/if}
</details>

<hr />
<StreetView {map} bind:enabled={controls.streetviewOn} />
