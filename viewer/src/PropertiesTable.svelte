<script lang="ts">
  import { calculate } from "lts";

  export let properties: { [name: string]: any };

  let tags = structuredClone(properties);
  // TODO Organize the GJ output better in the first-place, nest OSM tags elsewhere
  delete tags.count;
  delete tags.lts;
  delete tags.node1;
  delete tags.node2;
  delete tags.way;

  let ltsResult = calculate({
    method: "bike_ottawa",
    tags,
  });
</script>

<b>Count: {properties.count.toFixed(2)}</b>
<b>LTS (Rust): {ltsResult.lts}</b>
<ul>
  {#each ltsResult.messages as msg}
    <li>{msg}</li>
  {/each}
</ul>
<table>
  <tbody>
    {#each Object.entries(properties) as [key, value]}
      <tr><td>{key}</td><td>{value}</td></tr>
    {/each}
  </tbody>
</table>

<style>
  table {
    border: solid;
  }
</style>
