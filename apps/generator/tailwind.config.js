/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'app-bg': '#1a1a2e',
        'app-secondary': '#16213e',
        'app-border': '#333',
        'app-primary': '#646cff',
        'app-primary-hover': '#535bf2',
        'app-error': '#ff6b6b',
        'app-success': '#51cf66',
      },
    },
  },
  plugins: [],
}
