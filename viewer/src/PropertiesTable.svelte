<script lang="ts">
  import { calculate } from "lts";

  export let properties: { [name: string]: any };

  // Get detailed LTS messages, if we preserved OSM tags
  let ltsMessages = ["No LTS info, because OSM tags were dropped"];
  let tags = {};
  if (properties.osm_tags) {
    // Remember maplibre compacts nested feature properties, so we have to parse as JSON!
    tags = JSON.parse(properties.osm_tags);
    let { messages } = calculate({
      method: "bike_ottawa",
      tags,
    });
    ltsMessages = messages;
  }
</script>

{#if properties.count}
  <div>Count: <b>{properties.count.toFixed(2)}</b></div>
{/if}
{#if properties.cost}
  <div>
    Cost: {properties.cost} (<b
      >{(properties.cost / properties.length).toFixed(2)}</b
    >x the length)
  </div>
{/if}
<div>Nearby amenities: <b>{properties.nearby_amenities}</b></div>
<div>LTS: <b>{properties.lts}</b></div>
<ul>
  {#each ltsMessages as msg}
    <li>{msg}</li>
  {/each}
</ul>
{#if properties.osm_tags}
  <table>
    <tbody>
      {#each Object.entries(tags) as [key, value]}
        <tr><td>{key}</td><td>{value}</td></tr>
      {/each}
    </tbody>
  </table>
{:else}
  <p>OSM tags weren't kept</p>
{/if}

<style>
  table {
    border: solid;
  }
</style>
