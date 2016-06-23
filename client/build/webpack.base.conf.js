var path = require('path')
var config = require('../config')
var utils = require('./utils')
var projectRoot = path.resolve(__dirname, '../')
var nodeModules = path.join(projectRoot, 'node_modules')

module.exports = {
	entry: {
		app: './src/main.js'
	},
	output: {
		path: config.build.assetsRoot,
		publicPath: config.build.assetsPublicPath,
		filename: '[name].js'
	},
	resolve: {
		extensions: ['', '.js', '.vue'],
		fallback: [nodeModules],
		alias: {
			'src': path.resolve(__dirname, '../src'),
			'assets': path.resolve(__dirname, '../src/assets'),
			'components': path.resolve(__dirname, '../src/components')
		}
	},
	resolveLoader: {
		fallback: [nodeModules]
	},
	module: {
		preLoaders: [
		{
			test: /\.vue$/,
			loader: 'eslint',
			include: projectRoot,
			exclude: /node_modules/
		},
		{
			test: /\.js$/,
			loader: 'eslint',
			include: projectRoot,
			exclude: /node_modules/
		}
		],
		loaders: [
		{
			test: /\.vue$/,
			loader: 'vue'
		},
		{
			test: /\.js$/,
			loader: 'babel',
			include: projectRoot,
			exclude: /node_modules/
		},
		{
			test: /\.json$/,
			loader: 'json'
		},
		{
			test: /\.html$/,
			loader: 'vue-html'
		},
		{
			test: /\.(png|jpe?g|gif|svg)(\?.*)?$/,
			loader: 'url',
			query: {
				limit: 10000,
				name: utils.assetsPath('img/[name].[hash:7].[ext]')
			}
		},
		{
			test: /\.(woff2?|eot|ttf|otf)(\?.*)?$/,
			loader: 'url',
			query: {
				limit: 10000,
				name: utils.assetsPath('fonts/[name].[hash:7].[ext]')
			}
		}
		]
	},
	postcss: function(webpack) {
		return [
			autoprefixer({browsers: ['last 2 versions', 'ie >= 9', 'and_chr >= 2.3']})
		]
	},
	sassLoader: {
		includePaths: [nodeModules]
	},
	eslint: {
		formatter: require('eslint-friendly-formatter')
	},
	vue: {
		loaders: utils.cssLoaders()
	}
}
