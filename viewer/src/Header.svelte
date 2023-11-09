<script lang="ts">
  import logoUrl from "../assets/logo.png?url";

  // TODO Merge apps into one page, or detect what the user's loading?
  export let app: string;

  let dialog;

  function switchApp() {
    if (app == "main") {
      window.location.href = "index.html";
    } else if (app == "interactive") {
      window.location.href = "interactive.html";
    } else if (app == "costs") {
      window.location.href = "edge_cost.html";
    }
  }
</script>

<div style="display: flex; justify-content: space-between">
  <img src={logoUrl} width="100" alt="od2net logo by Hadrien Salat" />

  <select bind:value={app} on:change={switchApp}>
    <option value="main">Pre-built route networks</option>
    <option value="interactive">Interactively generate route networks</option>
    <option value="costs">Explore edge costs</option>
  </select>
</div>

<dialog bind:this={dialog}>
  <div style="display: flex; justify-content: space-between">
    <h1>About od2net</h1>
    <button autofocus on:click={() => dialog.close()}>Close</button>
  </div>
  <img src={logoUrl} width="300" alt="od2net logo by Hadrien Salat" />

  <p>
    od2net is a project by <a href="https://dcarlino.org" target="_blank"
      >Dustin Carlino</a
    >, part of the
    <a
      href="https://urban-analytics-technology-platform.github.io"
      target="_blank">Urban Analytics Technology Platform</a
    >. It wouldn't be possible without:
  </p>
  <ul>
    <li><a href="https://georust.org" target="_blank">GeoRust</a></li>
    <li>
      <a href="https://github.com/dimfeld/svelte-maplibre/" target="_blank"
        >svelte-maplibre</a
      >
    </li>
    <li><a href="https://maplibre.org" target="_blank">MapLibre GL JS</a></li>
    <li>
      <a
        href="https://www.npmjs.com/package/chartjs-plugin-dragdata"
        target="_blank">ChartJS</a
      >
    </li>
    <li>
      <a href="https://www.openstreetmap.org/about" target="_blank"
        >OpenStreetMap</a
      >
    </li>
    <li>
      <a href="https://github.com/easbar/fast_paths" target="_blank"
        >fast-paths</a
      >
    </li>
    <li>
      <a href="https://pct.bike" target="_blank">Propensity to Cycle Tool</a>
    </li>
    <li>
      <a href="https://protomaps.com/" target="_blank">PMTiles</a>
    </li>
    <li>
      Special thanks to
      <a href="https://github.com/HSalat/" target="_blank">Hadrien Salat</a> for
      the logo and
      <a href="https://github.com/stuartlynn/" target="_blank">Stuart Lynn</a>
      for feedback and help with pmtiles, both from
      <a href="https://www.turing.ac.uk/" target="_blank"
        >the Alan Turing Institute</a
      >
    </li>
  </ul>
</dialog>
<button on:click={() => dialog.showModal()}>About</button>

<br />
<br />

<style>
  ::backdrop {
    background-color: black;
    opacity: 0.5;
  }
</style>
