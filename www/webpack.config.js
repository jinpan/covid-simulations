const CopyWebpackPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require('path');

module.exports = {
  mode: "production",
  // mode: "development",
  entry: {
    index: "./bootstrap.js",
    intro: "./src/intro_bootstrap.js",
    shopping_solo: "./src/shopping_solo_bootstrap.js",
    shopping_with_masks: "./src/shopping_with_masks_bootstrap.js",
  },
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
      template: 'src/shopping_with_masks.html',
      chunks: ['shopping_with_masks'],
      filename: './shopping_with_masks/index.html',
    }),
  ],
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "./[name].bundle.js",
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
    ],
  },
};
