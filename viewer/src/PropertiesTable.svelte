<script lang="ts">
  export let properties: { [name: string]: any };
</script>

<b>Count: {properties.count}</b>
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
