/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {},
    borderWidth: {
      DEFAULT: '1px',
      0: '0',
      2: '1.5px',
      3: '3px',
      4: '4px',
      8: '8px',
    },
  },
  plugins: [],
}

