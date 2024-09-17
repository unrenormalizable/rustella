import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import tailwindcss from 'tailwindcss'
import { ViteRsw } from 'vite-plugin-rsw'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), ViteRsw()],
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
