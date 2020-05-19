const CopyWebpackPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require('path');

module.exports = {
  entry: {
    index: "./bootstrap.js",
    intro: "./bootstrap_intro.js",
    shopping_solo: "./bootstrap_shopping_solo.js",
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "./[name].bundle.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html']),
    new HtmlWebpackPlugin({
      hash: true,
      template: 'intro.html',
      chunks: ['intro'],
      filename: './intro/index.html',
    }),
    new HtmlWebpackPlugin({
      hash: true,
      template: 'shopping_solo.html',
      chunks: ['shopping_solo'],
      filename: './shopping_solo/index.html',
    }),
  ],
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
    ],
  },
};
