<script lang="ts">
  import type { Map } from "maplibre-gl";

  export let layer: string;
  export let map: Map;
  // TODO It'd ve great to detect this with getLayoutProperty, but the layer is
  // usually not finished loading by the time we create this component
  export let show: boolean;

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
