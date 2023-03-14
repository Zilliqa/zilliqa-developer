const fs = require('fs')
const webpack = require('webpack');
const packageJson = fs.readFileSync('./package.json')
const version = JSON.parse(packageJson).version || 0

module.exports = {
    css: {
        loaderOptions: {
            sass: {
                prependData: `@import "@/styles/main.scss";`
            }
        }
    },
    configureWebpack: {
        plugins: [
            new webpack.DefinePlugin({
                'process.env': {
                    PACKAGE_VERSION: '"' + version + '"'
                }
            })
        ]
    },
    // the rest of your original module.exports code goes here
}