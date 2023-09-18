// This is adapted from https://raw.githubusercontent.com/BikeOttawa/stressmodel/master/stressmodel.js, MIT licensed

// Call evaluateLTS with way object that looks like: {id:'id', tags:{'highway': 'residential', 'maxspeed':'40', 'lanes':'2'}}
// This will return an object {lts:2 message:['line 1','line 2', ...,'last line']}
// where message contains a list of the decisions made to determine the LTS level of the specified way.
export function evaluateLTS(way) {
  const bp = bikingPermitted(way);
  if (!bp.permitted) {
    return bp.result;
  }
  const isp = isSeparatedPath(way);
  if (isp.isSeparatedPath) {
    return isp.result;
  }
  const ibl = isBikeLane(way);
  if (ibl.isBikeLane) {
    return ibl.result;
  }
  const imt = isMixedTraffic(way);
  if (imt.isMixedTraffic) {
    return imt.result;
  }

  return {
    lts: 0,
    message: ["Error: This way does not match any of the analysis criteria."],
  };
}

function HasTag(way, tag) {
  return typeof way.tags[tag] !== "undefined";
}

function HasTagValue(way, tag, tagvalue) {
  let x = way.tags[tag];
  if (typeof x === "undefined") return false;
  return x === tagvalue;
}

function TagStartsWith(way, tag) {
  for (let t in way.tags) {
    if (t.startsWith(tag)) return true;
  }
  return false;
}

function TagStartsWithValue(way, tag, tagvalue) {
  for (let t in way.tags) {
    if (t.startsWith(tag)) {
      if (way.tags[t] === tagvalue) return true;
    }
  }
  return false;
}

function getLanes(way) {
  let lanes;
  let defaultLanes = 2;
  if (HasTag(way, "lanes")) {
    let l = way.tags["lanes"];
    if (l.indexOf(";") === -1) {
      lanes = parseInt(l);
      if (isNaN(lanes)) {
        return {
          lanes: defaultLanes,
          message: [
            "Error: Unknown 'lanes' tag: '" +
              l +
              "'. Assuming default of " +
              defaultLanes.toString() +
              ".",
          ],
        };
      } else {
        return { lanes: lanes, message: [] };
      }
    } else {
      let list = l.split(";");
      let ltot = 0;
      for (let s in list) {
        lanes = parseInt(s);
        if (isNaN(lanes)) {
          return {
            lanes: defaultLanes,
            message: [
              "Error: Unknown 'lanes' tag in split value: '" +
                l +
                "'. Assuming default of " +
                defaultLanes.toString() +
                ".",
            ],
          };
        } else {
          ltot += lanes;
        }
      }
      return {
        lanes: ltot,
        message: ["Split 'lanes' tag. Using total of all elements."],
      };
    }
  } else {
    return {
      lanes: defaultLanes,
      message: [
        "No 'lanes' tag. Assuming default of " + defaultLanes.toString() + ".",
      ],
    };
  }
}

function getMaxSpeed(way) {
  let result;
  if (HasTag(way, "maxspeed")) {
    const maxspeed = way.tags["maxspeed"];
    if (maxspeed === "national") {
      return {
        maxspeed: 40,
        message: [
          "Error: Unknown 'maxspeed' tag value 'national'. Assuming a value of '40'.",
        ],
      };
    } else {
      if (maxspeed.endsWith(" mph")) {
        result = parseInt(maxspeed.slice(0, -4)) * 1.60934;
      } else {
        result = parseInt(maxspeed);
      }
      if (isNaN(result)) {
        return {
          maxspeed: 50,
          message: [
            "Error: Unknown 'maxspeed' tag value '" +
              maxspeed +
              "'. Assuming a value of '50'.",
          ],
        };
      } else {
        return { maxspeed: result, message: [] };
      }
    }
  } else {
    if (HasTagValue(way, "highway", "motorway")) {
      return {
        maxspeed: 100,
        message: [
          "No maxspeed tag. Using default value of 100 when highway=motorway.",
        ],
      };
    } else if (
      HasTagValue(way, "highway", "primary") ||
      HasTagValue(way, "highway", "secondary")
    ) {
      return {
        maxspeed: 80,
        message: [
          "No maxspeed tag. Using default value of 80 when highway='" +
            way.tags["highway"] +
            "'.",
        ],
      };
    } else {
      return {
        maxspeed: 50,
        message: [
          "No maxspeed tag. Using default value of 50 when highway='" +
            way.tags["highway"] +
            "'.",
        ],
      };
    }
  }
}

function parkingPresent(way) {
  if (HasTagValue(way, "parking", "yes")) {
    return {
      parking: true,
      message: ["Found tag 'parking'='yes'. Parking is present."],
    };
  }
  if (TagStartsWith(way, "parking:")) {
    for (let tag in way.tags) {
      if (tag.startsWith("parking:lane")) {
        const v = way.tags[tag];
        if (
          v === "parallel" ||
          v === "perpendicular" ||
          v === "diagonal" ||
          v === "yes" ||
          v === "marked"
        ) {
          return {
            parking: true,
            message: [
              "Found tag '" + tag + "'='" + v + "'. Parking is present.",
            ],
          };
        }
      }
    }
  }
  return { parking: false, message: ["Parking is not present."] };
}

function hasSeparatingMedian(way) {
  return false;
}

function bikeAndParkingWidth(way) {
  // FIXME: This is the sum of bike and parking lane width. It includes
  // marked buffer and paved gutter. We currently can't count it so we
  // just assume the maximum to remove the effect from the calculation.
  // This is a placeholder until such time as we can model this component.
  return 99999.9;
}

function bikingPermitted(way) {
  if (HasTag(way, "highway") || HasTag(way, "bicycle")) {
    if (HasTagValue(way, "bicycle", "no")) {
      return {
        permitted: false,
        result: {
          lts: 0,
          message: ["Cycling not permitted due to bicycle='no' tag."],
          rule: "p2",
        },
      };
    }
    if (HasTagValue(way, "access", "no")) {
      return {
        permitted: false,
        result: {
          lts: 0,
          message: ["Cycling not permitted due to access='no' tag."],
          rule: "p6",
        },
      };
    }
    if (HasTagValue(way, "highway", "motorway")) {
      return {
        permitted: false,
        result: {
          lts: 0,
          message: ["Cycling not permitted due to highway='motorway' tag."],
          rule: "p3",
        },
      };
    } else if (HasTagValue(way, "highway", "motorway_link")) {
      return {
        permitted: false,
        result: {
          lts: 0,
          message: [
            "Cycling not permitted due to highway='motorway_link' tag.",
          ],
          rule: "p4",
        },
      };
    } else if (HasTagValue(way, "highway", "proposed")) {
      return {
        permitted: false,
        result: {
          lts: 0,
          message: ["Cycling not permitted due to highway='proposed' tag."],
          rule: "p7",
        },
      };
    }
    if (HasTagValue(way, "footway", "sidewalk")) {
      if (!HasTagValue(way, "bicycle", "yes")) {
        if (
          HasTagValue(way, "highway", "footway") ||
          HasTagValue(way, "highway", "path")
        ) {
          return {
            permitted: false,
            result: {
              lts: 0,
              message: [
                "Cycling not permitted. When footway='sidewalk' is present, there must be a bicycle='yes' when the highway is 'footway' or 'path'.",
              ],
              rule: "p5",
            },
          };
        }
      }
    }
  } else {
    return {
      permitted: false,
      result: {
        lts: 0,
        message: [
          "Way has neither a highway tag nor a bicycle=yes tag. The way is not a highway.",
        ],
        rule: "p1",
      },
    };
  }

  return { permitted: true, message: [] };
}

function isSeparatedPath(way) {
  let rule = "";
  let message = [];
  if (HasTagValue(way, "highway", "path")) {
    rule = "s1";
    message.push("This way is a separated path because highway='path'.");
  } else if (
    HasTagValue(way, "highway", "footway") &&
    !HasTagValue(way, "footway", "crossing")
  ) {
    rule = "s2";
    message.push(
      "This way is a separated path because highway='footway' but it is not a crossing."
    );
  } else if (HasTagValue(way, "highway", "cycleway")) {
    rule = "s3";
    message.push("This way is a separated path because highway='cycleway'.");
  } else if (HasTagValue(way, "highway", "construction")) {
    if (HasTagValue(way, "construction", "path")) {
      rule = "s4";
      message.push(
        "This way is a separated path because highway='construction' and construction='path'."
      );
    } else if (HasTagValue(way, "construction", "footway")) {
      rule = "s5";
      message.push(
        "This way is a separated path because highway='construction' and construction='footway'."
      );
    } else if (HasTagValue(way, "construction", "cycleway")) {
      rule = "s6";
      message.push(
        "This way is a separated path because highway='construction' and construction='cycleway'."
      );
    }
  } else if (TagStartsWithValue(way, "cycleway", "track")) {
    rule = "s7";
    message.push(
      "This way is a separated path because cycleway* is defined as 'track'."
    );
  } else if (TagStartsWithValue(way, "cycleway", "opposite_track")) {
    rule = "s8";
    message.push(
      "This way is a separated path because cycleway* is defined as 'opposite_track'."
    );
  }
  if (rule) {
    message.push("Separated paths are always LTS=1.");
    return {
      isSeparatedPath: true,
      result: { lts: 1, message: message, rule: rule },
    };
  }

  return { isSeparatedPath: false };
}

function bikeLaneAnalysisParkingPresent(way, message) {
  let rule = "";
  const isResidential = HasTagValue(way, "highway", "residential");
  const width = bikeAndParkingWidth(way);

  const gl = getLanes(way);
  const lanes = gl.lanes;
  if (gl.message.length > 0) {
    message.push(gl.message);
  }

  const gm = getMaxSpeed(way);
  const maxspeed = gm.maxspeed;
  if (gm.message.length > 0) {
    message.push(gm.message);
  }

  let lts = 1;
  if (lanes >= 3) {
    if (lts < 3) {
      rule = "b2";
      message.push(
        "Increasing LTS to 3 because there are 3 or more lanes and parking present."
      );
      lts = 3;
    }
  }

  if (width <= 4.1) {
    if (lts < 3) {
      rule = "b3";
      message.push(
        "Increasing LTS to 3 because the bike lane width is less than 4.1m and parking present."
      );
      lts = 3;
    }
  } else if (width <= 4.25) {
    if (lts < 2) {
      rule = "b4";
      message.push(
        "Increasing LTS to 2 because the bike lane width is less than 4.25m and parking present."
      );
      lts = 2;
    }
  } else if (width < 4.5 && (maxspeed < 40 || isResidential)) {
    if (lts < 2) {
      rule = "b5";
      message.push(
        "Increasing LTS to 2 because the bike lane width is less than 4.5m. maxspeed is less than 40 on a residential street and parking present."
      );
      lts = 2;
    }
  }

  if (maxspeed > 40) {
    if (maxspeed <= 50) {
      if (lts < 2) {
        rule = "b6";
        message.push(
          "Increasing LTS to 2 because the maxspeed is between 41-50 km/h and parking present."
        );
        lts = 2;
      }
    } else if (maxspeed < 65) {
      if (lts < 3) {
        rule = "b7";
        message.push(
          "Increasing LTS to 3 because the maxspeed is between 51-54 km/h and parking present."
        );
        lts = 3;
      }
    } else {
      if (lts < 4) {
        rule = "b8";
        message.push(
          "Increasing LTS to 4 because the maxspeed is over 55 km/h and parking present."
        );
        lts = 4;
      }
    }
  }
  if (!isResidential) {
    if (lts < 3) {
      rule = "b9";
      message.push("Increasing LTS to 3 because highway is not 'residential'.");
      lts = 3;
    }
  }
  if (lts === 1) {
    rule = "b1";
    message.push(
      "LTS is 1 because there is parking present, the maxspeed is less than or equal to 40, highway='residential', and there are 2 lanes or less."
    );
  }

  return { lts: lts, message: message, rule: rule };
}

function bikeLaneAnalysisNoParking(way, message) {
  let rule = "";
  const isResidential = HasTagValue(way, "highway", "residential");
  const width = bikeAndParkingWidth(way);

  const gl = getLanes(way);
  const lanes = gl.lanes;
  if (gl.message.length > 0) {
    message.push(gl.message);
  }

  const gm = getMaxSpeed(way);
  const maxspeed = gm.maxspeed;
  if (gm.message.length > 0) {
    message.push(gm.message);
  }

  let lts = 1;
  if (lanes === 3 && hasSeparatingMedian(way)) {
    if (lts < 2) {
      rule = "c2";
      message.push(
        "Increasing LTS to 2 because there are 3 lanes with a separating median and no parking."
      );
      lts = 2;
    }
  } else if (lanes >= 3) {
    if (lts < 3) {
      rule = "c3";
      message.push(
        "Increasing LTS to 3 because there are 3 or more lanes and no parking."
      );
      lts = 3;
    }
  }

  if (width <= 1.7) {
    if (lts < 2) {
      rule = "c4";
      message.push(
        "Increasing LTS to 2 because the bike lane width is less than 1.7 metres and no parking."
      );
      lts = 2;
    }
  }
  if (maxspeed > 50) {
    if (maxspeed < 65) {
      if (lts < 3) {
        rule = "c5";
        message.push(
          "Increasing LTS to 3 because the maxspeed is between 51-64 km/h and no parking."
        );
        lts = 3;
      }
    } else {
      if (lts < 4) {
        rule = "c6";
        message.push(
          "Increasing LTS to 4 because the maxspeed is over 65 km/h and no parking."
        );
        lts = 4;
      }
    }
  }
  if (!isResidential) {
    if (lts < 3) {
      rule = "c7";
      message.push(
        "Increasing LTS to 3 because highway with bike lane is not 'residential' and no parking."
      );
      lts = 3;
    }
  }
  if (lts === 1) {
    rule = "c1";
    message.push(
      "LTS is 1 because there is no parking, maxspeed is less than or equal to 50, highway='residential', and there are 2 lanes or less."
    );
  }

  return { lts: lts, message: message, rule: rule };
}

function isBikeLane(way) {
  let result;
  let analyze = false;
  let message = [];
  if (
    TagStartsWithValue(way, "cycleway", "crossing") ||
    TagStartsWithValue(way, "cycleway", "lane") ||
    TagStartsWithValue(way, "cycleway", "left") ||
    TagStartsWithValue(way, "cycleway", "opposite") ||
    TagStartsWithValue(way, "cycleway", "opposite_lane") ||
    TagStartsWithValue(way, "cycleway", "right") ||
    TagStartsWithValue(way, "cycleway", "yes")
  ) {
    analyze = true;
    for (let t in way.tags) {
      if (t.startsWith("cycleway")) {
        message.push(
          "Way has a bike lane because '" + t + "'='" + way.tags[t] + "'."
        );
      }
    }
  }
  if (HasTagValue(way, "shoulder:access:bicycle", "yes")) {
    analyze = true;
    message.push(
      "Way has a bike lane because shoulder:access:bicycle='" +
        way.tags["shoulder:access:bicycle"] +
        "'."
    );
  }
  if (analyze) {
    const pp = parkingPresent(way);
    if (pp.message.length > 0) message.push(pp.message);
    if (pp.parking) {
      result = bikeLaneAnalysisParkingPresent(way, message);
      return { isBikeLane: true, result: result };
    } else {
      result = bikeLaneAnalysisNoParking(way, message);
      return { isBikeLane: true, result: result };
    }
  }

  return { isBikeLane: false };
}

function isMixedTraffic(way) {
  let message = [
    "Does not meet criteria for Separated Path or Bike Lane. Treating as Mixed Traffic.",
  ];

  const isResidential = HasTagValue(way, "highway", "residential");

  const gl = getLanes(way);
  const lanes = gl.lanes;
  if (gl.message.length > 0) {
    message.push(gl.message);
  }

  const gm = getMaxSpeed(way);
  let maxspeed = gm.maxspeed;
  if (gm.message.length > 0) {
    message.push(gm.message);
  }

  if (HasTagValue(way, "motor_vehicle", "no")) {
    message.push("Setting LTS to 1 because motor_vehicle='no'.");
    return {
      isMixedTraffic: true,
      result: { lts: 1, message: message, rule: "m17" },
    };
  }
  if (HasTagValue(way, "highway", "steps")) {
    message.push("Setting LTS to 1 because highway='steps'.");
    return {
      isMixedTraffic: true,
      result: { lts: 1, message: message, rule: "m1" },
    };
  }
  if (HasTagValue(way, "highway", "pedestrian")) {
    message.push("Setting LTS to 1 because highway='pedestrian'.");
    return {
      isMixedTraffic: true,
      result: { lts: 1, message: message, rule: "m13" },
    };
  }
  if (
    HasTagValue(way, "highway", "footway") &&
    HasTagValue(way, "footway", "crossing")
  ) {
    message.push(
      "Setting LTS to 2 because highway='footway' and footway='crossing'."
    );
    return {
      isMixedTraffic: true,
      result: { lts: 2, message: message, rule: "m14" },
    };
  }
  if (
    HasTagValue(way, "highway", "service") &&
    HasTagValue(way, "service", "alley")
  ) {
    message.push(
      "Setting LTS to 2 because highway='service' and service='alley'."
    );
    return {
      isMixedTraffic: true,
      result: { lts: 2, message: message, rule: "m2" },
    };
  }
  if (HasTagValue(way, "highway", "track")) {
    message.push("Setting LTS to 2 because highway='track'.");
    return {
      isMixedTraffic: true,
      result: { lts: 2, message: message, rule: "m15" },
    };
  }
  if (maxspeed <= 50) {
    if (HasTagValue(way, "highway", "service")) {
      if (HasTagValue(way, "service", "parking_aisle")) {
        message.push(
          "Setting LTS to 2 because maxspeed is 50 km/h or less and service is 'parking_aisle'."
        );
        return {
          isMixedTraffic: true,
          result: { lts: 2, message: message, rule: "m3" },
        };
      }
      if (HasTagValue(way, "service", "driveway")) {
        message.push(
          "Setting LTS to 2 because maxspeed is 50 km/h or less and service is 'driveway'."
        );
        return {
          isMixedTraffic: true,
          result: { lts: 2, message: message, rule: "m4" },
        };
      }
      if (maxspeed < 35) {
        message.push(
          "Setting LTS to 2 because maxspeed is less than 35 km/h and highway='service'."
        );
        return {
          isMixedTraffic: true,
          result: { lts: 2, message: message, rule: "m16" },
        };
      }
    }
    if (maxspeed <= 40) {
      if (lanes <= 3 && isResidential) {
        message.push(
          "Setting LTS to 2 because maxspeed is up to 40 km/h, 3 or fewer lanes and highway='residential'."
        );
        return {
          isMixedTraffic: true,
          result: { lts: 2, message: message, rule: "m5" },
        };
      } else if (lanes <= 3) {
        message.push(
          "Setting LTS to 3 because maxspeed is up to 40 km/h and 3 or fewer lanes on non-residential highway."
        );
        return {
          isMixedTraffic: true,
          result: { lts: 3, message: message, rule: "m6" },
        };
      } else if (lanes <= 5) {
        message.push(
          "Setting LTS to 3 because maxspeed is up to 40 km/h and 4 or 5 lanes."
        );
        return {
          isMixedTraffic: true,
          result: { lts: 3, message: message, rule: "m7" },
        };
      } else {
        message.push(
          "Setting LTS to 4 because maxspeed is up to 40 km/h and the number of lanes is greater than 5."
        );
        return {
          isMixedTraffic: true,
          result: { lts: 4, message: message, rule: "m8" },
        };
      }
    } else {
      if (lanes < 3 && isResidential) {
        message.push(
          "Setting LTS to 2 because maxspeed is up to 50 km/h and lanes are 2 or less and highway='residential'."
        );
        return {
          isMixedTraffic: true,
          result: { lts: 2, message: message, rule: "m9" },
        };
      } else if (lanes <= 3) {
        message.push(
          "Setting LTS to 3 because maxspeed is up to 50 km/h and lanes are 3 or less on non-residential highway."
        );
        return {
          isMixedTraffic: true,
          result: { lts: 3, message: message, rule: "m10" },
        };
      } else {
        message.push(
          "Setting LTS to 4 because the number of lanes is greater than 3."
        );
        return {
          isMixedTraffic: true,
          result: { lts: 4, message: message, rule: "m11" },
        };
      }
    }
  } else {
    message.push("Setting LTS to 4 because maxspeed is greater than 50 km/h.");
    return {
      isMixedTraffic: true,
      result: { lts: 4, message: message, rule: "m12" },
    };
  }
}
