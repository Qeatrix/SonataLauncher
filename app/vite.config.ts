import { defineConfig } from 'vite'
import path from 'node:path'
import { resolve } from "node:path"

// https://vitejs.dev/config/
export default defineConfig({
  esbuild: {
    jsx: "automatic",
    jsxImportSource: "hywer",
  },
  target: ["esnext"],
  cssMinify: "lightningcss",
  minify: "terser",
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
      "#public": resolve(__dirname, "public"),
      "#root": resolve(__dirname)
    }
  },
  css: {
    preprocessorOptions: {
      less: {
        paths: [
          path.resolve(__dirname, "src"),
        ]
      },
    },
  },
})
