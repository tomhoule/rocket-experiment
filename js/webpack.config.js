const path = require('path');

module.exports = {
    entry: {
        'edition-new': './js/src/edition-new.ts',
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].js', // switch to '[name]__[hash].js' when we can register handlers in Rocket
    },
    resolve: {
        extensions: ['.js', '.ts', '.tsx'],
    },
    devtool: 'source-map',
    module: {
        rules: [
            { test: /.tsx?/, use: 'awesome-typescript-loader' },
        ],
    },
}
