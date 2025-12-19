export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        mono: ['"JetBrains Mono"', '"Roboto Mono"', 'monospace'],
      },
      colors: {
        primary: {
          blue: '#3b82f6', // Scan/Add
          green: '#22c55e', // Pay/Confirm
          red: '#ef4444', // Delete/Void
        }
      }
    },
  },
  plugins: [],
}
