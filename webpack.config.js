const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = (env, argv) => {
    const isProduction = (argv.mode === 'production');

    return {
        entry: './index.js',
        output: {
            path: path.resolve(__dirname, 'dist'),
            filename: isProduction ? '[name].[contenthash].js' : '[name].[hash].js',
            publicPath: '/',
        },
        devServer: {
            static: {
                directory: path.join(__dirname, '.'), // Source of your assets
                watch: true, // This replaces the watchContentBase option
            },
            hot: true,  // Enable hot module replacement
            historyApiFallback: true,  // For SPA routing
            port: 9000
        },
        experiments: {
            asyncWebAssembly: true
        },
        plugins: [
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, '.')
            }),
            new HtmlWebpackPlugin({
                template: './index.html'
            }),
            new webpack.ProvidePlugin({
                TextDecoder: ['text-encoding', 'TextDecoder'],
                TextEncoder: ['text-encoding', 'TextEncoder']
            }),
        ]
    };
};