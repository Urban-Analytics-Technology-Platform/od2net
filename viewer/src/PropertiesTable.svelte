<script lang="ts">
  import { evaluateLTS } from "./lts";

  export let properties: { [name: string]: any };

  let result = evaluateLTS({ tags: properties });
</script>

<b>Count: {properties.count}</b>
<ul>
  {#each result.message as msg}
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
