<script lang="ts">
  import Plotly from "plotly.js-dist";

  export let title: string;
  export let data: number[];

  function histogram(node) {
    Plotly.purge(node);
    Plotly.newPlot(node, [{ x: data, type: "histogram" }], { title });

    return {
      /*update({ dataset: newData }) {
        Plotly.react(node, [{ x: newData, type: "histogram" }], { title });
      },*/
      destroy() {
        Plotly.purge(node);
      },
    };
  }
</script>

<div use:histogram />
