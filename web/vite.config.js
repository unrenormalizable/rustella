import path from 'node:path'
import { defineConfig, normalizePath } from 'vite'
import react from '@vitejs/plugin-react-swc'
import tailwindcss from 'tailwindcss'
import { ViteRsw } from 'vite-plugin-rsw'
import { viteStaticCopy } from 'vite-plugin-static-copy'

export default defineConfig({
  plugins: [
    react(),
    ViteRsw(),
    viteStaticCopy({
      targets: [
        {
          src: normalizePath(
            path.resolve(__dirname, `../emu/tests/bins/collect/*.bin`)
          ),
          dest: `./roms/`,
        },
        {
          src: normalizePath(
            path.resolve(__dirname, `../emu/tests/bins/*.bin`)
          ),
          dest: `./roms/`,
        },
      ],
    }),
  ],
  css: {
    postcss: {
      plugins: [tailwindcss()],
    },
  },
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['src/__tests__/setup.js'],
  },
})
