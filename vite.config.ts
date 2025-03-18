import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [solid(), wasm(), topLevelAwait()],
  /*build: {
    rollupOptions: {
      output: {
        dir: path.resolve('./dist'),
        entryFileNames: 'bundle.js',
        format: 'iife',
      }
    }
  },*/
  worker: {
    plugins: () => [wasm(), topLevelAwait()],
    format: "es",
  }
})
