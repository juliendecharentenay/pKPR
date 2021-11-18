module.exports = {
  // css: {
    // extract: false,
  // },
  // configureWebpack: {
    // output: {
      // filename: "index.js",
      // chunkFilename: "index-chunk.js"
    // }
  // },
  configureWebpack: {
	  optimization: {
		  splitChunks: false
	  }
  },
	filenameHashing: false
}
