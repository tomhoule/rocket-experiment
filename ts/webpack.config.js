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
            { test: /.tsx?/, use: 'awesome-typescript-loader' },
        ],
    },
}
