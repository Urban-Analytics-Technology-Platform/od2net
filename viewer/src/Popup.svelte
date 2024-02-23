<script lang="ts">
  // Use this component instead of the one from svelte-maplibre, to more
  // conveniently get the properties of the topmost feature in a TS-friendly
  // way.
  import type { Feature } from "geojson";
  import { Popup } from "svelte-maplibre";

  // TODO Maybe set openIfTopMost and other props, or pass them through
  export let openOn: "hover" | "click" = "hover";

  function getProperties(features: Feature[] | null): { [name: string]: any } {
    if (!features) {
      console.log("A Popup with null features should be impossible");
      return {};
    }
    return features[0].properties ?? {};
  }
</script>

<Popup {openOn} let:features>
  <slot props={getProperties(features)} />
</Popup>
