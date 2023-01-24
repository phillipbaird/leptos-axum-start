/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: {
    files: ["*.html", "./app/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [],
}
