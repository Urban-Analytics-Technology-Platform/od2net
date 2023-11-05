import type { DataDrivenPropertyValueSpecification } from "maplibre-gl";

export let colors = {
  origins: "blue",
  destinations: "purple",

  // Colors from https://github.com/BikeOttawa/maps.bikeottawa.ca-frontend/blob/master/lts/index.html
  lts1: "#009e73",
  lts2: "#56b4e9",
  lts3: "#e69f00",
  lts4: "#d55e00",
  lts_unknown: "black",
};

export let colorByLts = [
  "match",
  ["get", "lts"],
  1,
  colors.lts1,
  2,
  colors.lts2,
  3,
  colors.lts3,
  4,
  colors.lts4,
  colors.lts_unknown,
];

// Helper for https://maplibre.org/maplibre-style-spec/expressions/#step.
export function makeColorRamp(
  input: DataDrivenPropertyValueSpecification<number>,
  limits: number[],
  colorScale: string[]
): DataDrivenPropertyValueSpecification<string> {
  let step: any[] = ["step", input];
  for (let i = 1; i < limits.length; i++) {
    step.push(colorScale[i - 1]);
    step.push(limits[i]);
  }
  // Repeat the last color. The upper limit is exclusive, meaning a value
  // exactly equal to it will use this fallback. For things like percentages,
  // we want to set 100 as the cap.
  step.push(colorScale[colorScale.length - 1]);
  return step as DataDrivenPropertyValueSpecification<string>;
}

// Sequential (low-to-high) color ramp from https://www.ons.gov.uk/census/maps/choropleth
export let colorScale = ["#CDE594", "#80C6A3", "#1F9EB7", "#186290", "#080C54"];
