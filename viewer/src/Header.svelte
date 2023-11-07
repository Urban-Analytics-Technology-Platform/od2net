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
  <sp-close-button autofocus on:click={() => dialog.close()}
    >Close</sp-close-button
  >
  <p>TODO: Write about. Creidts, part of TP, etc</p>
</dialog>
<sp-button on:click={() => dialog.showModal()}>About</sp-button>

<br />
<br />

<style>
  ::backdrop {
    background-color: black;
    opacity: 0.5;
  }
</style>
