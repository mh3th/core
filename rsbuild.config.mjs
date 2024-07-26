import { defineConfig } from '@rsbuild/core';
export default defineConfig({
    source: {
        entry: {
            index: './web',
        },
    },
    html: {
        template: './web/index.html',
        filename: 'index.html',
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
