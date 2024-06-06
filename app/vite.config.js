import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
  esbuild: {
    jsx: "automatic",
    jsxImportSource: "hywer",
  },
  base: "src/",
  server: {
    port: 6969,
  }
})
