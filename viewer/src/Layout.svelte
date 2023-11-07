<div class="left">
  <slot name="left" />
</div>
<div class="main">
  <slot name="main" />
</div>

<style>
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
  }
  .main {
    width: 75%;
  }
</style>
