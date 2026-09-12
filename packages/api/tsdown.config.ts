import { wasm } from "rolldown-plugin-wasm";
import { defineConfig } from "tsdown";

export default defineConfig({
  deps: {
    neverBundle: ["@turbo-napi/node", "@turbo-napi/web"],
  },
  dts: true,
  entry: {
    node: "src/node.ts",
    web: "src/web.ts",
  },
  format: "esm",
  plugins: [wasm()],
  sourcemap: false,
  tsconfig: "tsconfig.json",
});
