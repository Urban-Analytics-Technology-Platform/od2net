<script lang="ts">
  import { ltsNames } from "./common";
  import TradeoffRadar from "./TradeoffRadar.svelte";

  export let cost;

  // TODO Maybe not in sync with what's passed in initially
  let costChoice = "Distance";

  let generalized = {
    tradeoff_lts: 50,
    tradeoff_amenities: 30,
    tradeoff_greenspace: 20,

    lts1: 0.0,
    lts2: 0.2,
    lts3: 0.6,
    lts4: 1.0,

    minimum_amenities: 1,
  };

  let ltsWeights = {
    lts1: 1.0,
    lts2: 1.0,
    lts3: 1.0,
    lts4: 1.0,
  };

  // TODO Let people add/remove choices
  let osmHighwayWeights = {};
  for (let key of [
    "cycleway",
    "footway",
    "living_street",
    "motorway",
    "motorway_link",
    "path",
    "pedestrian",
    "primary",
    "primary_link",
    "residential",
    "secondary",
    "secondary_link",
    "service",
    "steps",
    "tertiary",
    "tertiary_link",
    "track",
    "trunk",
    "trunk_link",
    "unclassified",
  ]) {
    osmHighwayWeights[key] = 1.0;
  }

  function setCost(costChoice) {
    if (costChoice == "OsmHighwayType") {
      cost = { OsmHighwayType: osmHighwayWeights };
    } else if (costChoice == "ByLTS") {
      cost = { ByLTS: ltsWeights };
    } else if (costChoice == "Generalized") {
      cost = { Generalized: generalized };
    } else {
      cost = costChoice;
    }
  }
  $: setCost(costChoice);
</script>

<div>
  <label>
    Cost function:
    <select bind:value={costChoice}>
      <option value="Distance">Distance</option>
      <option value="Generalized">Generalized cost function</option>
      <option value="ByLTS">Weight per LTS</option>
      <option value="OsmHighwayType">Set a weight per OSM highway type</option>
    </select>
  </label>
</div>

{#if costChoice == "OsmHighwayType"}
  <ul>
    {#each Object.keys(osmHighwayWeights) as key}
      <li>
        <label
          >{key}<input
            type="number"
            min="1.0"
            step="0.1"
            bind:value={osmHighwayWeights[key]}
            on:change={() => (cost = cost)}
          /></label
        >
      </li>
    {/each}
  </ul>
{:else if costChoice == "ByLTS"}
  <ul>
    {#each ["lts1", "lts2", "lts3", "lts4"] as key}
      <li>
        <label
          >{ltsNames[key]}<input
            type="number"
            min="1.0"
            step="0.1"
            bind:value={ltsWeights[key]}
            on:change={() => (cost = cost)}
          /></label
        >
      </li>
    {/each}
  </ul>
{:else if costChoice == "Generalized"}
  <TradeoffRadar
    bind:lts={generalized.tradeoff_lts}
    bind:nearbyAmenities={generalized.tradeoff_amenities}
    bind:greenspace={generalized.tradeoff_greenspace}
  />
  <ul>
    {#each ["lts1", "lts2", "lts3", "lts4"] as key}
      <li style="display: flex; justify-content: space-between;">
        {ltsNames[key]}: {generalized[key]}
        <input
          type="range"
          min="0.0"
          max="1.0"
          step="0.1"
          bind:value={generalized[key]}
        />
      </li>
    {/each}
  </ul>

  <div>
    <label
      >Give roads a good score if they have at least this many nearby amenities:
      <input
        type="number"
        min="0"
        bind:value={generalized.minimum_amenities}
      /></label
    >
  </div>

  <button type="button" on:click={() => (cost = cost)}>Recalculate</button>
{/if}
