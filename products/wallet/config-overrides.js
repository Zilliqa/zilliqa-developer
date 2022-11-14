const WorkerPlugin = require("worker-plugin");
const webpack = require("webpack");

module.exports = function override(config, env) {
  //  config.plugins.push(new WorkerPlugin());
  /*
    config.resolve.fallback = {
      ...config.resolve.fallback,
      stream: require.resolve("stream-browserify"),
      buffer: require.resolve("buffer"),
    };

    config.resolve.extensions = [
      ...config.resolve.extensions,
      ".ts",
      ".js",
      ".tsx",
      ".jsx",
    ];
    config.plugins = [
      ...config.plugins,
      new webpack.ProvidePlugin({
        Buffer: ["buffer", "Buffer"],
      }),
      new webpack.ProvidePlugin({
        process: "process/browser",
      }),
    ];
  */

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

  config.plugins.push(new WorkerPlugin());
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

  return config;
};
