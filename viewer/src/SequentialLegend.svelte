<script lang="ts">
  export let colorScale: string[];
  export let limits: any[];
</script>

<div style="display: flex">
  {#each colorScale as color}
    <span style="background: {color}; width: 100%; border: 1px solid black;">
      &nbsp;
    </span>
  {/each}
</div>
<div style="display: flex; justify-content: space-between;">
  {#each limits as limit}
    <span>{limit}</span>
  {/each}
</div>
