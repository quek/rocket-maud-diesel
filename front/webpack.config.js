var path = require('path');

module.exports = {
  entry: {
    aaa: './src/index.js',
    posts: './src/posts.js'
  },
  output: {
    // ilename: '[name].[chunkhash].js', // production!!!
    filename: '[name].js',
    path: path.resolve(__dirname, 'dist')
  }
};
