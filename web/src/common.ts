import type { ExpressionSpecification } from "maplibre-gl";

export interface LayersControls {
  maxCount: number;
  originRadius: number;
  destinationRadius: number;
  streetviewOn: boolean;
}

export type Cost =
  | "Distance"
  | {
      ByLTS: {
        lts1: number;
        lts2: number;
        lts3: number;
        lts4: number;
      };
    }
  | { OsmHighwayType: { [key: string]: any } }
  | {
      Generalized: {
        tradeoff_lts: number;
        tradeoff_amenities: number;
        tradeoff_greenspace: number;

        lts1: number;
        lts2: number;
        lts3: number;
        lts4: number;

        minimum_amenities: number;
      };
    };

export let colors = {
  origins: "blue",
  destinations: "purple",

  // Colors from https://github.com/BikeOttawa/maps.bikeottawa.ca-frontend/blob/master/lts/index.html
  lts1: "#009e73",
  lts2: "#56b4e9",
  lts3: "#e69f00",
  lts4: "#d55e00",
  lts_not_allowed: "red",
};

// TODO constructMatchExpression isn't working for some reason; numeric keys?
export let colorByLts: ExpressionSpecification = [
  "match",
  ["get", "lts"],
  0,
  colors.lts_not_allowed,
  1,
  colors.lts1,
  2,
  colors.lts2,
  3,
  colors.lts3,
  4,
  colors.lts4,
  // Shouldn't happen
  "red",
];

// Sequential (low-to-high) color ramp from https://www.ons.gov.uk/census/maps/choropleth
export let colorScale = ["#CDE594", "#80C6A3", "#1F9EB7", "#186290", "#080C54"];

export let ltsNames = {
  lts1: "LTS 1 - suitable for children",
  lts2: "LTS 2 - low stress",
  lts3: "LTS 3 - medium stress",
  lts4: "LTS 4 - high stress",
  lts_not_allowed: "Cyclists not allowed",
};

// Thanks to https://ropensci.github.io/slopes/articles/roadnetworkcycling.html
export let slopeLimits = [0, 3, 5, 8, 10, 20, 100];
export let slopeColors = [
  "#267300",
  "#70A800",
  "#FFAA00",
  "#E60000",
  "#A80000",
  "#730000",
];
