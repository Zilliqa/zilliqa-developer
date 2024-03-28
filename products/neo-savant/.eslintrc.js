module.exports = {
  root: true,
  env: {
    node: true,
    es2022: true,
  },
  extends: ["plugin:vue/essential", "eslint:recommended"],
  rules: {
    "no-console": import.meta.NODE_ENV === "production" ? "error" : "off",
    "no-debugger": import.meta.NODE_ENV === "production" ? "error" : "off",
  },
};
