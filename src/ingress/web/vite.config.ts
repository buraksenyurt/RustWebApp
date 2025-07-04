import {defineConfig} from 'vite';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
    plugins: [vue()],
    build: {
        outDir: 'dist',
        rollupOptions: {
            input: './index.html',
            output: {
                entryFileNames: `assets/index.js`,
                chunkFileNames: `assets/chunk.js`,
                assetFileNames: `assets/[name][extname]`
            }
        }
    }
});
