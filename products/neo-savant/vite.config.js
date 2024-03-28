// vite.config.js

import { defineConfig } from 'vite'
import { createVuePlugin as vue } from "vite-plugin-vue2";
const path = require("path");

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue()],
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
            `
          }
        }
    }
})