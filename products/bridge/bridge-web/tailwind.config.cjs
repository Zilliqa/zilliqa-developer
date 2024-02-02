/* eslint-disable no-undef */
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    fontFamily: {
      sans: ["montserrat"],
    },
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        mytheme: {
          primary: "#2B9297",
          "primary-content": "#ffffff",
          secondary: "#1e1e1e",
          accent: "#1e1e1e",
          neutral: "#1e1e1e",
          "neutral-content": "#ffffff",
          content: "#ffffff",
          "base-100": "#010101",
          // info: "#ffffff",
          // success: "#00ffff",
          // warning: "#ffffff",
          // error: "#ffffff",
          "--rounded-box": "5px",
        },
      },
    ],
  },
};
