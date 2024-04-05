// vite.config.js

import { defineConfig } from "vite";
import { createVuePlugin as vue } from "vite-plugin-vue2";
import { nodePolyfills } from "vite-plugin-node-polyfills";

const path = require("path");

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    nodePolyfills({
      globals: {
        Buffer: true, // can also be 'build', 'dev', or false
      },
    }),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  assetsInclude: ["src/contracts/*.scilla"],
  css: {
    preprocessorOptions: {
      scss: {
        additionalData: `
            @import "@/styles/main.scss";
            `,
      },
    },
  },
});
