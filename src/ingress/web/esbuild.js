const esbuild = require('esbuild');
const vuePlugin = require('esbuild-plugin-vue');

esbuild.build({
    plugins: [vuePlugin()],
    entryPoints: ['src/main.ts'],
    bundle: true,
    outfile: 'public/bundle.js',
    format: 'esm',
    define: {
        'process.env.NODE_ENV': '"production"',
    },
    minify: true,
    sourcemap: true,
    loader: {
        '.ts': 'ts',
        '.vue': 'ts',
        '.css': 'css'
    },
}).catch(() => process.exit(1));
