import { writable } from "svelte/store";

export let showRouteNetwork = writable(true);
export let showOrigins = writable(false);
export let showDestinations = writable(false);
