const CopyWebpackPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require('path');

module.exports = {
  entry: {
    index: "./bootstrap.js",
    intro: "./src/intro_bootstrap.js",
    shopping_solo: "./src/shopping_solo_bootstrap.js",
    shopping_bulk: "./src/shopping_bulk_bootstrap.js",
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "./[name].bundle.js",
  },
  // mode: "production",
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html']),
    new HtmlWebpackPlugin({
      hash: true,
      template: 'src/intro.html',
      chunks: ['intro'],
      filename: './intro/index.html',
    }),
    new HtmlWebpackPlugin({
      hash: true,
      template: 'src/shopping_solo.html',
      chunks: ['shopping_solo'],
      filename: './shopping_solo/index.html',
    }),
    new HtmlWebpackPlugin({
      hash: true,
      template: 'src/shopping_bulk.html',
      chunks: ['shopping_bulk'],
      filename: './shopping_bulk/index.html',
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
