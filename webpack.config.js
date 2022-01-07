const path = require("path");
const { VueLoaderPlugin } = require("vue-loader");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const { ModuleFederationPlugin } = require("webpack").container;
const pkg = require("./package.json");

module.exports = (env = {}) => ({
  name: pkg.config.shortname,
  mode: "production",
  cache: false,
  devtool: "source-map",
  optimization: {
    minimize: false,
  },
  target: "web",
  entry: path.resolve(__dirname, "./src/index.js"),
  output: {
    publicPath: !env.prod? "http://localhost:3002/": pkg.config.publicPath,
  },
  resolve: {
    extensions: [".vue", ".jsx", ".js", ".json"],
    alias: {
      vue: "@vue/runtime-dom",
    },
  },
  module: {
    rules: [
      {
        test: /\.vue$/,
        use: "vue-loader",
      },
      {
        test: /\.svg$/i,
        use: {
          loader: "url-loader",
          options: { limit: 8192 },
        },
      },
      {
        test: /\.png$/,
        use: {
          loader: "url-loader",
          options: { limit: 8192 },
        },
      },
      {
        test: /\.css$/,
        use: [
          {
            loader: MiniCssExtractPlugin.loader,
          },
          "css-loader",
        ],
      },
    ],
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: "[name].css",
    }),
    new ModuleFederationPlugin({
      name: pkg.config.shortname,
      library: { type: "amd", name: pkg.config.shortname },
      filename: "remoteEntry.js",
      remotes: {
        "lenna-web": "lenna-web",
      },
      exposes: {
        "default": "./src/",
        "./Widget": "./src/Widget",
      },
      remotes: {},
      shared: ['vue']
    }),
    new VueLoaderPlugin(),
  ],
  experiments: {
    syncWebAssembly: true,
  },
  devServer: {
    contentBase: path.join(__dirname),
    compress: true,
    port: 3002,
    hot: true,
    headers: {
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, PATCH, OPTIONS",
      "Access-Control-Allow-Headers":
        "X-Requested-With, content-type, Authorization",
    },
  },
});
