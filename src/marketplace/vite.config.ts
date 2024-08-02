/// <reference types="vitest" />
import svgr from 'vite-plugin-svgr';
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import environment from 'vite-plugin-environment';
import dotenv from 'dotenv';

dotenv.config();

export default defineConfig({
  server: {
    port: 3000
  },
  build: {
    outDir: 'build',
  },
  plugins: [
    react(),
    svgr({
      svgrOptions: {
        icon: true,
      }
      
    }),
    environment('all', { prefix: 'CANISTER_' }),
    environment('all', { prefix: 'DFX_' }),
  ],
  resolve: {
    alias: {
        app: '/src/app',
        entities: '/src/entities',
        features: '/src/features',
        pages: '/src/pages',
        shared: '/src/shared',
        widgets: '/src/widgets',
        process: 'process/browser'
    },
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: 'globalThis',
      },
    },
  },
})