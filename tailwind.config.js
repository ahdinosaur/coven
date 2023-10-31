/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [],
  theme: {
    extend: {},
  },
  safelist: [
    // HACK: render ALL tailwind and daisyui classes
    // TODO: use build.rs to render dioxus app to html in dist, then pass dist into tailwind to check classes
    { pattern: /./ },
  ],
  plugins: [require("daisyui")],
};
