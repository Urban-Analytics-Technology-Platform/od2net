<script lang="ts">
  export let example: string;
</script>

<label>
  Or load an example:
  <select bind:value={example}>
    <option value="">Custom file loaded</option>
    <option value="antwerp">Antwerp</option>
    <option value="berlin">Berlin</option>
    <option value="london">South London</option>
    <option value="paris">Paris</option>
    <option value="seattle">Seattle</option>
  </select>
</label>
