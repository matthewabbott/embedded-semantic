import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  build: {
    target: 'esnext',
  },
  server: {
    fs: {
      // Allow serving files from parent directory
      allow: ['..']
    }
  }
})