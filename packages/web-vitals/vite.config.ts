import {resolve} from 'path'
import {defineConfig} from 'vite'

export default defineConfig({
  build: {
    minify: 'esbuild',
    lib: {
      name: 'webVitals',
      entry: resolve(__dirname, 'lib/index.js'),
      formats: ['umd'],
    },
  },
})
