const path = require('path');

module.exports = {
  entry: './ts/src/main.tsx',
  // 'edition-new': './js/src/edition-new.ts',
  // },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js', // switch to '[name]__[hash].js' when we can register handlers in Rocket
  },
  resolve: {
    extensions: ['.js', '.ts', '.tsx'],
      modules: ['./node_modules', './ts/src'],
  },
  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.scss$/,
        use: [
          'style-loader',
          {
            loader: 'css-loader',
            options: {
              modules: true,
              importLoaders: 1,
              localIdentName: '[local]___[hash:base64:5]',
            },
          },
          'sass-loader',
        ],
      },
      {
        test: /\.js$/,
        use: [
          {
            loader: 'babel-loader',
            options: {
              presets: [
                ['env', { targets: { browsers: ['IE 9'] } }],
              ],
            },
          },
        ],
      },
      {
        test: /\.tsx?$/,
        use: [
          {
            loader: 'babel-loader',
            options: {
              presets: [
                ['env', { targets: { browsers: ['IE 9'] } }],
              ],
            },
          },
          { loader: 'awesome-typescript-loader' },
        ],
      },
    ],
  },
  devServer: {
    // contentBase: path.join(__dirname, 'public'), // boolean | string | array, static file location
    // compress: true, // enable gzip compression
    historyApiFallback: true, // true for index.html upon 404, object for multiple paths
    // hot: true, // hot module replacement. Depends on HotModuleReplacementPlugin
  },
}
