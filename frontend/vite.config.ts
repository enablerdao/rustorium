import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://localhost:9071',
        changeOrigin: true,
      },
      '/ws': {
        target: 'ws://localhost:9072',
        ws: true,
      },
    },
  },
});
