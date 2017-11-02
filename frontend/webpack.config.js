const path = require('path');

module.exports = {
  entry: {
    'wholebundle': './frontend/src/main.ts',
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js', // switch to '[name]__[hash].js' when we can register handlers in Rocket
  },
  resolve: {
    extensions: ['.js', '.ts', '.tsx'],
    modules: ['./node_modules', './frontend/src'],
  },
  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.s[ac]ss$/,
        use: [
          'style-loader',
          {
            loader: 'css-loader',
            options: {
              modules: false,
              importLoaders: 1,
            },
          },
          {
            loader: 'sass-loader',
            options: {
              includePaths: ["node_modules"]
            }
          }
        ],
      },
      {
        test: /\.js$/,
        use: [
          {
            loader: 'babel-loader',
            options: {
              presets: [
                ['env', { targets: { browsers: ['IE 11'] } }],
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
                ['env', { targets: { browsers: ['IE 11'] } }],
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
