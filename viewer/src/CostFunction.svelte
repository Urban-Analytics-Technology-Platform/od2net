<script lang="ts">
  // TODO Widget's broken, bind:value not working
  export let cost;

  // TODO Maybe not in sync with what's passed in initially
  let costChoice = "Distance";
  // TODO Let people add/remove choices
  let osmHighwayWeights = new Map(
    [
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
    ].map((k) => [k, 1.0])
  );

  function setCost(costChoice) {
    if (costChoice == "OsmHighwayType") {
      cost = { OsmHighwayType: osmHighwayWeights };
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
      <option value="AvoidMainRoads">Avoid main roads</option>
      <option value="OsmHighwayType">Set a weight per OSM highway type</option>
    </select>
  </label>
</div>
{#if costChoice == "OsmHighwayType"}
  <ul>
    {#each osmHighwayWeights.entries() as [key, value] (key)}
      <li>
        <label
          >{key}<input type="number" min="1.0" step="0.1" bind:value /></label
        >
      </li>
    {/each}
  </ul>
{/if}
