import svgr from 'vite-plugin-svgr';
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path';
import env from 'vite-plugin-env-compatible';
import { NodeGlobalsPolyfillPlugin } from '@esbuild-plugins/node-globals-polyfill';
import { NodeModulesPolyfillPlugin } from '@esbuild-plugins/node-modules-polyfill';
import environment from 'vite-plugin-environment';

export default defineConfig({
  plugins: [
    react(),
    svgr({
      svgrOptions: {
        icon: true,
      }
      
    }),
    env(),
    environment('all', { prefix: 'CANISTER_' }),
    environment('all', { prefix: 'DFX_' }),
  ],
  server: {
    port: 3000
  },
  // root: 'public',
  build: {
    outDir: 'build',
  },
  define: {
    global: {},
    process: {}
  },
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
      // Node.js global to browser global polyfills
      define: {
        global: 'globalThis'
      },
      plugins: [
        NodeGlobalsPolyfillPlugin(),
        NodeModulesPolyfillPlugin()
      ]
    }
  }
})