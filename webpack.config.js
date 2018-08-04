const CleanWebpackPlugin = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

const DIST_DIR = path.resolve(__dirname, "dist");
const IS_PROD = process.env.NODE_ENV === "production";

console.log("PRODUCTION?", IS_PROD);

const config = {
    mode: IS_PROD ? "production" : "development",
    //devtool: IS_PROD ? undefined : "inline-source-map",
    entry: "./target/deploy/eosstrawpoll.js",
    node: {
        fs: "empty"
    },
    output: {
        path: DIST_DIR,
        filename: "index.js",
        publicPath: "/",
    },
    performance: {
        hints: false,
    },
    devServer: {
        contentBase: [
            DIST_DIR,
            "./target/deploy/",
        ],
        compress: true,
        port: 9000,
        historyApiFallback: true,
    },
    module: {
        rules: [
            {
                test: /eosstrawpoll\.js$/,
                loader: 'string-replace-loader',
                options: {
                    search: 'fetch( "eosstrawpoll.wasm"',
                    replace: 'fetch( "/index.wasm"',
                }
            }
        ]
    },
    plugins: [
        new CleanWebpackPlugin(["dist"], {
            verbose: true,
        }),
        new HtmlWebpackPlugin({
            template: "src/index.html",
            filename: "index.html",
        }),
        new CopyWebpackPlugin([{
            from: "./target/deploy/eosstrawpoll.wasm",
            to: "index.wasm"
        }]),
    ],
};

module.exports = config;