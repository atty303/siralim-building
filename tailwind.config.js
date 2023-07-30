/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        primary: "#D0BCFF",
        // "on-primary": "#381E72",
      }
    },
  },
  plugins: [require("daisyui")],
}

