import { defineConfig } from '@rsbuild/core';
export default defineConfig({
    output: {
        distPath: {
            html: '../src/presentation/views/pages'
        }
    },
    source: {
        entry: {
            index: './src/presentation/views/main.js',
        },
    },
    html: {
        template: './src/presentation/views/template.html',
        inject: 'body',
    },
    tools: {
        postcss: {
            postcssOptions: {
                plugins: [
                    require('tailwindcss'),
                ],
            },
        }
    },
});
