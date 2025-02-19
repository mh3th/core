const plugin = require('tailwindcss/plugin');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/presentation/views/**/*.html'],
  theme: {
    extend: {
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    plugin(function ({ addVariant }) {
      addVariant('htmx-settling', ['&.htmx-settling', '.htmx-settling &'])
      addVariant('htmx-request', ['&.htmx-request', '.htmx-request &'])
      addVariant('htmx-swapping', ['&.htmx-swapping', '.htmx-swapping &'])
      addVariant('htmx-added', ['&.htmx-added', '.htmx-added &'])
    }),
  ],
};