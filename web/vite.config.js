import path from 'node:path'
import { defineConfig, normalizePath } from 'vite'
import react from '@vitejs/plugin-react-swc'
import tailwindcss from 'tailwindcss'
import { ViteRsw } from 'vite-plugin-rsw'
import { viteStaticCopy } from 'vite-plugin-static-copy'

const targets = [...Array(2).keys()].map((i) => ({
  src: normalizePath(
    path.resolve(
      __dirname,
      `../emu/tests/spiceware_collect/${i + 1}/collect.bin`
    )
  ),
  dest: `./${i + 1}`,
}))

export default defineConfig({
  plugins: [
    react(),
    ViteRsw(),
    viteStaticCopy({
      targets,
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
