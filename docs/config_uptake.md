# Scoring route likelihood

Even if a route is perfectly safe, it might be unlikely somebody would use it just based on the total distance or hilliness. The configurable "uptake model" assigns a probability between 0 and 1 to every route, and this value is summed for each edge.

The possible values for `"uptake"`:

- `"identity"` -- every route counts as 1, equivalent to just counting every trip
- `{ "CutoffMaxDistanceMeters": 16000 }` -- trips over 16km are skipped entirely, otherwise they count as 1
- `"GovTargetPCT"` and `"GoDutchPCT"` are uptake models from the PCT, using distance and gradient (**currently hardcoded to 0**)
