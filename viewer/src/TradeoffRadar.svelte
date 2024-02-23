<script lang="ts">
  // TODO Mostly giving up on TS in here
  import { Chart, registerables } from "chart.js";
  // @ts-ignore No types
  import ChartJSdragDataPlugin from "chartjs-plugin-dragdata";
  import { createEventDispatcher } from "svelte";

  Chart.register(...registerables);
  Chart.register(ChartJSdragDataPlugin);

  export let lts: number;
  export let nearbyAmenities: number;
  export let greenspace: number;

  const dispatch = createEventDispatcher<{
    // TODO void
    change: string;
  }>();

  let barChart: Chart | null = null;
  let colors = ["red", "blue", "green"];

  normalize();

  function normalize() {
    let sum = lts + nearbyAmenities + greenspace;
    lts = (100 * lts) / sum;
    nearbyAmenities = (100 * nearbyAmenities) / sum;
    greenspace = (100 * greenspace) / sum;

    if (barChart) {
      barChart.data.datasets[0].data = [lts, nearbyAmenities, greenspace];
      barChart.update();
    }
  }

  function makeRadarChart(node: HTMLCanvasElement) {
    let options = {
      type: "radar" as const,
      data: {
        labels: ["LTS", "Amenities", "Greenspace"],
        datasets: [
          {
            label: "Routing preferences",
            data: [lts, nearbyAmenities, greenspace],
            pointHitRadius: 25,
          },
        ],
      },
      options: {
        responsive: false,
        onHover: function (e: any) {
          const point = e.chart.getElementsAtEventForMode(
            e,
            "nearest",
            { intersect: true },
            false,
          );
          e.native.target.style.cursor = point.length ? "grab" : "default";
        },
        plugins: {
          dragData: {
            round: 1,
            showTooltip: true,
            onDragStart: function (e: any) {
              e.target.style.cursor = "grabbing";
            },
            onDragEnd: function (
              e: any,
              datasetIndex: number,
              index: number,
              value: any,
            ) {
              e.target.style.cursor = "default";
              if (index == 0) {
                lts = value;
              } else if (index == 1) {
                nearbyAmenities = value;
              } else if (index == 2) {
                greenspace = value;
              }

              normalize();
              dispatch("change", "");
            },
          },
        },
        scales: {
          r: {
            min: 0,
            max: 100,
            stepSize: 1,
            pointLabels: {
              color: colors,
              font: {
                weight: "bold",
                size: 15,
              },
            },
            ticks: {
              display: false,
            },
          },
        },
      },
    };
    // @ts-ignore dragData isn't recognized
    new Chart(node.getContext("2d")!, options);
  }

  function makeBarChart(node: HTMLCanvasElement) {
    let options = {
      type: "bar" as const,
      data: {
        labels: ["LTS", "Amenities", "Greenspace"],
        datasets: [
          {
            label: "Routing preferences",
            data: [lts, nearbyAmenities, greenspace],
            backgroundColor: colors,
          },
        ],
      },
      options: {
        indexAxis: "y" as const,
      },
    };
    barChart = new Chart(node.getContext("2d")!, options);
  }
</script>

<canvas use:makeRadarChart style="width: 100%; height: 400px;" />
<canvas use:makeBarChart style="width: 100%; height: 300px;" />
