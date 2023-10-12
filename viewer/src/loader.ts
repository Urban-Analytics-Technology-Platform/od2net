import { PMTiles } from "pmtiles";

export interface Info {
  bounds: [number, number, number, number];
  outputMetadata: any;
  minZoom: number;
  maxZoom: number;
}

export async function pmtilesInfo(file: PMTiles): Promise<Info> {
  let header = await file.getHeader();
  let metadata = await file.getMetadata();

  return {
    bounds: [header.minLon, header.minLat, header.maxLon, header.maxLat],
    outputMetadata: JSON.parse(metadata.description),
    minZoom: header.minZoom,
    maxZoom: header.maxZoom,
  };
}
