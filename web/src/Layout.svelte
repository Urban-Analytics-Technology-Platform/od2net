<div class="left">
  <slot name="left" />
</div>
<div class="main">
  <slot name="main" />
</div>

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
