/* eslint-disable */
// const WorkerPlugin = require("worker-plugin");
/* eslint-disable */
const webpack = require("webpack");

module.exports = function override(config) {
  /*
  config.resolve.fallback = {
    url: require.resolve("url"),
    assert: require.resolve("assert"),
    crypto: require.resolve("crypto-browserify"),
    http: require.resolve("stream-http"),
    https: require.resolve("https-browserify"),
    os: require.resolve("os-browserify/browser"),
    buffer: require.resolve("buffer"),
    stream: require.resolve("stream-browserify"),
    string_decoder: require.resolve("string_decoder"),
    events: require.resolve("events"),
  };
  */

  config.plugins.push(
    new webpack.ProvidePlugin({
      process: "process/browser",
    })
  );
  config.plugins.push(
    new webpack.ProvidePlugin({
      Buffer: ["buffer", "Buffer"],
    })
  );

  // Due to REACT 18 + WEBPACK 5: Error: Can't resolve 'process/browser'
  // Patch according to https://github.com/react-dnd/react-dnd/issues/3425
  config.module.rules.unshift({
    test: /\.m?js$/,
    resolve: {
      fullySpecified: false, // disable the behavior
    },
  });
  return config;
};
