<script lang="ts">
  import MapboxDraw from "@mapbox/mapbox-gl-draw";
  import type { Feature, Polygon } from "geojson";
  import type { IControl, LngLat, Map } from "maplibre-gl";
  import { createEventDispatcher, onDestroy } from "svelte";
  import "@mapbox/mapbox-gl-draw/dist/mapbox-gl-draw.css";
  import type { OsmSelection } from "./types";

  export let map: Map | null;

  const dispatch = createEventDispatcher<{
    loading: string;
    gotXml: string;
    error: string;
  }>();

  let drawControls: MapboxDraw | null = null;

  $: if (map && !drawControls) {
    // TODO Hack from https://github.com/maplibre/maplibre-gl-js/issues/2601.
    // Remove dependency on this entirely.
    // @ts-ignore
    MapboxDraw.constants.classes.CONTROL_BASE = "maplibregl-ctrl";
    // @ts-ignore
    MapboxDraw.constants.classes.CONTROL_PREFIX = "maplibregl-ctrl-";
    // @ts-ignore
    MapboxDraw.constants.classes.CONTROL_GROUP = "maplibregl-ctrl-group";

    drawControls = new MapboxDraw({
      displayControlsDefault: false,
      controls: {
        polygon: true,
      },
    });
    // Hack around TS errors that don't matter at runtime
    map.addControl(drawControls as unknown as IControl);

    map.on("draw.create", async (e) => {
      let boundaryGj = e.features[0];
      drawControls!.deleteAll();
      await importPolygon(boundaryGj);
    });
  }

  onDestroy(() => {
    if (map?.loaded() && drawControls) {
      map.removeControl(drawControls as unknown as IControl);
    }
  });

  async function importPolygon(boundaryGj: Feature<Polygon>) {
    try {
      dispatch("loading", "Loading from Overpass");
      let resp = await fetch(overpassQueryForPolygon(boundaryGj));
      let osmXml = await resp.text();

      dispatch("gotXml", osmXml);
    } catch (err: any) {
      dispatch("error", err.toString());
    }
  }

  // Construct a query to extract all XML data in the polygon clip. See
  // https://wiki.openstreetmap.org/wiki/Overpass_API/Overpass_QL
  function overpassQueryForPolygon(feature: Feature<Polygon>): string {
    let filter = 'poly:"';
    for (let [lng, lat] of feature.geometry.coordinates[0]) {
      filter += `${lat} ${lng} `;
    }
    filter = filter.slice(0, -1) + '"';
    let query = `(nwr(${filter}); node(w)->.x; <;); out meta;`;
    return `https://overpass-api.de/api/interpreter?data=${query}`;
  }

  function latLngToGeojson(pt: LngLat): [number, number] {
    return [pt.lng, pt.lat];
  }

  // Turn the current viewport into a rectangular boundary
  function mapBoundsToGeojson(): Feature<Polygon> {
    let b = map!.getBounds();
    return {
      type: "Feature",
      properties: {},
      geometry: {
        coordinates: [
          [
            latLngToGeojson(b.getSouthWest()),
            latLngToGeojson(b.getNorthWest()),
            latLngToGeojson(b.getNorthEast()),
            latLngToGeojson(b.getSouthEast()),
            latLngToGeojson(b.getSouthWest()),
          ],
        ],
        type: "Polygon",
      },
    };
  }

  async function importCurrentView() {
    if (!map) {
      return;
    }
    if (map.getZoom() < 13) {
      dispatch("error", "Zoom in more to import");
      return;
    }
    await importPolygon(mapBoundsToGeojson());
  }
</script>

<button type="button" on:click={importCurrentView}>Import current view</button>

<style>
  /* TODO: These really do belong here, but getting a warning */

  :global(.mapboxgl-ctrl-group > button) {
    width: 60px;
    height: 60px;
  }

  :global(.mapbox-gl-draw_polygon) {
    background-size: 50px;
  }
</style>
