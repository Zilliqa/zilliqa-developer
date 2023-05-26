// rollup.config.js

import commonjs from "rollup-plugin-commonjs";
import builtins from "rollup-plugin-node-builtins";
import resolve from "rollup-plugin-node-resolve";
import { terser } from "rollup-plugin-terser";
import typescript from "rollup-plugin-typescript2";

const isProduction = process.env.NODE_ENV === "production";

export default {
  input: "src/index.ts",
  output: [
    {
      file: "dist/umd/bundle.js",
      format: "umd",
      name: "@zilliqa-js/util",
      sourcemap: !isProduction,
    },
    {
      file: "dist/esm/bundle.js",
      format: "esm",
      sourcemap: !isProduction,
    },
  ],
  plugins: [
    typescript(),
    resolve(),
    commonjs(),
    builtins(),
    isProduction && terser(),
  ],
};
