<script lang="ts">
  // This component manages a "pmtilesSource" source, based on the input.
  // TODO Upstream in svelte-maplibre and think through how to structure this more nicely.

  import type { Map as MapType } from "maplibre-gl";
  import maplibregl from "maplibre-gl";
  import { PMTiles, Protocol } from "pmtiles";

  // From a file or URL
  export let pmtiles: PMTiles | null;
  export let map: MapType;

  // Output
  export let outputMetadata: any | null = null;

  let source = "pmtilesSource";

  // One-time setup
  let protocol = new Protocol();
  maplibregl.addProtocol("pmtiles", protocol.tile);

  $: if (pmtiles) {
    setup(pmtiles);
  } else {
    cleanupSource(source);
    // TODO Clean up from the protocol?
  }

  async function setup(pmtiles: PMTiles) {
    // TODO Clean up from the protocol?
    cleanupSource(source);
    protocol.add(pmtiles);
    let info = await pmtilesInfo(pmtiles);

    map.addSource(source, {
      type: "vector",
      tiles: ["pmtiles://" + pmtiles.source.getKey() + "/{z}/{x}/{y}"],
      minzoom: info.minZoom,
      maxzoom: info.maxZoom,
      bounds: info.bounds,
    });
    map.fitBounds(info.bounds, { padding: 100, duration: 500 });

    outputMetadata = info.outputMetadata;
  }

  // TODO Overkill, now that this is localized in a component
  interface Info {
    bounds: [number, number, number, number];
    outputMetadata: any;
    minZoom: number;
    maxZoom: number;
  }

  async function pmtilesInfo(file: PMTiles): Promise<Info> {
    let header = await file.getHeader();
    let metadata = await file.getMetadata() as any;

    return {
      bounds: [header.minLon, header.minLat, header.maxLon, header.maxLat],
      outputMetadata: JSON.parse(metadata.description),
      minZoom: header.minZoom,
      maxZoom: header.maxZoom,
    };
  }

  function cleanupSource(id: string) {
    if (map.getSource(id)) {
      // First remove all layers using this source
      let layers = [];
      for (let layer of map.getStyle().layers) {
        if ("source" in layer && layer.source == id) {
          layers.push(layer.id);
        }
      }
      for (let layer of layers) {
        map.removeLayer(layer);
      }

      map.removeSource(id);
    }
  }
</script>
