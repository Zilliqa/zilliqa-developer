import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import polyfillNode from "rollup-plugin-polyfill-node";
import { terser } from "rollup-plugin-terser";
import typescript from "rollup-plugin-typescript2";

const isProduction = process.env.NODE_ENV === "production";

export default {
  input: "src/index.ts",
  output: [
    {
      file: "dist/umd/bundle.js",
      format: "umd",
      name: "@zilliqa-js/blockchain",
      sourcemap: !isProduction,
    },
  ],
  plugins: [
    typescript(),
    nodeResolve({
      browser: true,
    }),
    commonjs(),
    polyfillNode(),
    json(),
    isProduction && terser(),
  ],
};
