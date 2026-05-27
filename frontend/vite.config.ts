/// <reference types="vitest/config" />
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

// Removes crossorigin attributes from built HTML — Tauri's WebView
// doesn't set CORS headers on the custom protocol, causing scripts to fail.
function removeCrossorigin(): import('vite').Plugin {
  return {
    name: 'remove-crossorigin',
    enforce: 'post',
    transformIndexHtml(html) {
      return html.replaceAll(' crossorigin', '')
    },
  }
}

export default defineConfig({
  plugins: [react(), tailwindcss(), removeCrossorigin()],
  server: {
    strictPort: true,
  },
  build: {
    outDir: 'dist',
  },
  test: {
    globals: true,
    environment: 'jsdom',
    include: ['src/**/*.test.ts', 'src/**/*.test.tsx'],
    setupFiles: ['./src/test/setup.ts'],
  },
})
