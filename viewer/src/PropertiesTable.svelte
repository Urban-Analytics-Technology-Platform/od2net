<script lang="ts">
  import { calculate } from "lts";

  export let properties: { [name: string]: any };

  // Get detailed messages. Remember maplibre compacts nested feature properties, so we have to parse as JSON!
  let tags = JSON.parse(properties.osm_tags);
  let { messages } = calculate({
    method: "bike_ottawa",
    tags,
  });
</script>

<b>Count: {properties.count.toFixed(2)}</b>
<b>LTS: {properties.lts}</b>
<ul>
  {#each messages as msg}
    <li>{msg}</li>
  {/each}
</ul>
<table>
  <tbody>
    {#each Object.entries(tags) as [key, value]}
      <tr><td>{key}</td><td>{value}</td></tr>
    {/each}
  </tbody>
</table>

<style>
  table {
    border: solid;
  }
</style>
