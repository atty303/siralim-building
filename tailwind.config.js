/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["src/**/*.rs", "index.html"],
  theme: {
    extend: {
      colors: {
        death: "#6a1b9a",
        nature: "#558b2f",
        life: "#9e9d24",
        sorcery: "#283593",
        chaos: "#c62828",
      }
    },
  },
  plugins: [require("daisyui")],
}

