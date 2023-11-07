<script lang="ts">
  import '@spectrum-web-components/button/sp-button.js';
  import '@spectrum-web-components/button/sp-close-button.js';
import '@spectrum-web-components/theme/sp-theme.js';
import '@spectrum-web-components/theme/src/themes.js';
</script>

<sp-theme scale="large" color="light">
<div class="left">
  <slot name="left" />
</div>
<div class="main">
  <slot name="main" />
</div>
</sp-theme>

<style>
  * {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0px;
  }

  div {
    float: left;
  }

  .left {
    width: 25%;
    height: 100vh;
    overflow: scroll;
    padding: 8px;
  }
  .main {
    width: 75%;
  }
</style>
