const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './src/index.js',
  output: {
    filename: '[name].bundle.js',
    path: path.resolve(__dirname, 'dist'),
  },
  devServer: {
    static: path.resolve(__dirname, 'dist'),
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        {from: path.resolve(__dirname,'public/*.css'), to: '[name]', force: true},
        {from: path.resolve(__dirname,'public/img/**'), to: 'img', force: true}
      ]
    }, {debug: true}),
    new HtmlWebpackPlugin({
      title: 'Development',
      template: 'public/index.html'
    }),
  ],
  optimization: {
    runtimeChunk: 'single',
  },
  devtool: 'inline-source-map',
};