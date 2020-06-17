const webpack = require('webpack')
const wasmPackPlugin = require("@wasm-tool/wasm-pack-plugin")
const htmlWebpackPlugin = require("html-webpack-plugin") 
const path = require("path")
const copyPlugin = require('copy-webpack-plugin')

module.exports = (env, args) => {
    const isProductionMode = (args.mode === 'production')

    return  { 
        entry: './index.js',
        output: {
            //publicPath: ASSET_PATH,
            path: path.resolve(__dirname, 'dist-webpack'),
            filename: isProductionMode ? '[name].[contenthash].js':'[name].[hash].js',
            publicPath: '/'
        },
        plugins: [
            new copyPlugin({
                patterns: [
                    {from: 'public', to: 'public'}
                ]
            }),
            new htmlWebpackPlugin({
                template: 'index.html'
            }),
            new wasmPackPlugin({
                crateDirectory: path.resolve(__dirname, '.')
            }),
            new webpack.ProvidePlugin({
                TextDecoder: ['text-encoding', 'TextDecoder'],
                TextEncoder: ['text-encoding', 'TextEncoder']
            })
        ]

    }
}