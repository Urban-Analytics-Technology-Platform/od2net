<script lang="ts">
  import type { Map } from "maplibre-gl";

  export let layer: string;
  export let map: Map;

  let show = map.getLayoutProperty(layer, "visibility") == "visible";

  function toggle() {
    map.setLayoutProperty(layer, "visibility", show ? "visible" : "none");
  }
</script>

<div>
  <label>
    <input type="checkbox" bind:checked={show} on:change={toggle} />
    <slot />
  </label>
</div>
