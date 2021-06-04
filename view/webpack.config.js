const path = require('path')
const FileManagerPlugin = require('filemanager-webpack-plugin')
const { VueLoaderPlugin } = require("vue-loader")


module.exports = {
    entry: './src/index.js',
    output: {
        path: path.resolve(__dirname, './dest'),
        filename: 'bundle.js'
    },
    devServer: {
        contentBase: path.resolve(__dirname, 'public')
    },
    module: {
        rules: [
            {
                test: /\.vue$/,
                loader: 'vue-loader'
            },
            {
                test: /\.js$/,
                loader: 'babel-loader',
            },
            {
                test: /\.css$/,
                loader: 'style-loader!css-loader'
            },
            {
                test: /\.(jpg|png|otf|eot|svg|ttf|woff|woff2)(\?.+)?$/,
                loader: 'url-loader',
                options: {
                    name: '[name].[ext]?[hash]'
                }
            }
        ]
    },
    resolve: {
        extensions: ['.js', '.vue'],
        alias: {
            vue$: 'vue/dist/vue.esm.js',
        },
    },
    plugins: [
        new VueLoaderPlugin(),
        new FileManagerPlugin({
            events: {
                onEnd: {
                    copy: [
                        {
                            source: path.resolve(__dirname, 'dest', 'bundle.js'),
                            destination: path.resolve(__dirname, '../', 'src', 'bundle.js'),
                        }
                    ]
                }
            }
        })
    ],
}