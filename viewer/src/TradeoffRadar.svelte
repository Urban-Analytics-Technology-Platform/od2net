<script lang="ts">
  import { Chart, registerables } from "chart.js";
  import ChartJSdragDataPlugin from "chartjs-plugin-dragdata";

  Chart.register(...registerables);
  Chart.register(ChartJSdragDataPlugin);

  export let lts: number;
  export let nearbyAmenities: number;
  export let greenspace: number;

  let chart = null;
  let colors = ["red", "blue", "green"];

  normalize();

  function normalize() {
    let sum = lts + nearbyAmenities + greenspace;
    lts = (100 * lts) / sum;
    nearbyAmenities = (100 * nearbyAmenities) / sum;
    greenspace = (100 * greenspace) / sum;

    // TODO The area is usually small; it's less intuitive
    if (chart && false) {
      chart.data.datasets[0].data = [lts, nearbyAmenities, greenspace];
      chart.update();
    }
  }

  function makeChart(node) {
    let options = {
      type: "radar",
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
        onHover: function (e) {
          const point = e.chart.getElementsAtEventForMode(
            e,
            "nearest",
            { intersect: true },
            false
          );
          e.native.target.style.cursor = point.length ? "grab" : "default";
        },
        plugins: {
          dragData: {
            round: 1,
            showTooltip: true,
            onDragStart: function (e) {
              e.target.style.cursor = "grabbing";
            },
            onDragEnd: function (e, datasetIndex, index, value) {
              e.target.style.cursor = "default";
              if (index == 0) {
                lts = value;
              } else if (index == 1) {
                nearbyAmenities = value;
              } else if (index == 2) {
                greenspace = value;
              }

              normalize();
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
          },
        },
      },
    };
    chart = new Chart(node.getContext("2d"), options);
  }
</script>

<canvas use:makeChart style="width: 100%; height: 400px;" />
<ul>
  <li style:color={colors[0]}>LTS: {lts.toFixed(0)}%</li>
  <li style:color={colors[1]}>
    Nearby amenities: {nearbyAmenities.toFixed(0)}%
  </li>
  <li style:color={colors[2]}>
    Greenspace proximity: {greenspace.toFixed(0)}%
  </li>
</ul>
