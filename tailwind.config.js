/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./dist/expanded.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
};
