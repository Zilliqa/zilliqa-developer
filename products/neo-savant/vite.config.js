// vite.config.js

import { defineConfig } from "vite";
import createVuePlugin from "@vitejs/plugin-vue";
import { nodePolyfills } from "vite-plugin-node-polyfills";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import { ElementPlusResolver } from "unplugin-vue-components/resolvers";
import path from "path";

export default defineConfig({
  plugins: [
    createVuePlugin({
      template: {
        compilerOptions: {
          compatConfig: {
            MODE: 2,
          },
        },
      },
    }),
    nodePolyfills({
      globals: {
        Buffer: true, // can also be 'build', 'dev', or false
      },
    }),
    AutoImport({
      resolvers: [ElementPlusResolver()],
    }),
    Components({
      resolvers: [ElementPlusResolver()],
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
