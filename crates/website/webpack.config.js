const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const CompressionPlugin = require("compression-webpack-plugin");
const CleanWebpackPlugin = require("clean-webpack-plugin");
const path = require("path");
const webpack = require("webpack");
const globby = require("globby");

const IS_PROD = process.env.NODE_ENV === "production";
const DIST_DIR = path.resolve(__dirname, "..", "..", "docs");
const TARGET_DIR = path.resolve(
	__dirname,
	"..",
	"..",
	"target",
	"wasm32-unknown-unknown",
	"release",
);
const WASM_BINDGEN_DIR = path.resolve(TARGET_DIR, "wasm-bindgen");
const STATIC_DIR = path.resolve(__dirname, "static");

console.log("Settings:", {
	IS_PROD,
	DIST_DIR,
	TARGET_DIR,
	WASM_BINDGEN_DIR,
	STATIC_DIR,
});

function plugins() {
	const common = [
		new CleanWebpackPlugin([DIST_DIR]),
		new HtmlWebpackPlugin({
			template: path.resolve(STATIC_DIR, "index.html"),
			filename: "index.html",
		}),
	];
	if (IS_PROD) {
		return [
			...common,
			// new CopyWebpackPlugin([
			// 	{
			// 		from: TARGET_DIR + "website.wasm",
			// 		to: "index.wasm",
			// 	},
			// 	{
			// 		from: "./dist/index.css",
			// 		to: "index.css",
			// 	},
			// ]),
			new CompressionPlugin({
				test: /\.(html|css|js|wasm)$/,
			}),
		];
	} else {
		return [
			...common,
			// new CopyWebpackPlugin([
			// 	{
			// 		from: TARGET_DIR + "website.wasm",
			// 		to: "index.wasm",
			// 	},
			// ]),
		];
	}
}

const NODE_MODULES = path.resolve(__dirname, "node_modules");

const config = {
	mode: IS_PROD ? "production" : "development",
	entry: {
		index: path.resolve(STATIC_DIR, "index.js"),
	},
	node: {
		fs: "empty",
	},
	output: {
		path: DIST_DIR,
		filename: "[name].js",
		publicPath: "/",
	},
	resolve: {
		alias: {
			"wasm-bindgen": WASM_BINDGEN_DIR,
			"static-dir": STATIC_DIR,
		},
		modules: [NODE_MODULES],
	},
	optimization: {
		splitChunks: {
			cacheGroups: {
				commons: {
					test: /[\\/]node_modules[\\/]/,
					name: "vendor",
					chunks: "all",
				},
			},
		},
	},
	performance: {
		hints: "warning",
	},
	plugins: plugins(),
};

module.exports = config;
