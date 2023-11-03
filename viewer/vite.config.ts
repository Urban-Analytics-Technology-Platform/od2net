import { defineConfig } from "vite";
import { resolve } from "path";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasmPack from "vite-plugin-wasm-pack";

export default defineConfig({
  base: "/od2net/",
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        x2: resolve(__dirname, "interactive.html"),
        x3: resolve(__dirname, "edge_cost.html"),
      },
    },
  },
  plugins: [svelte(), wasmPack(["../lts", "../wasm-od2net"], [])]
})
