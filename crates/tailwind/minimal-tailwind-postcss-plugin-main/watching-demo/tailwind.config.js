module.exports = {
  content: [__dirname + '/{index,other}.html', __dirname + '/glob-example-folder/**/*.html'],
  theme: {
    colors: {
      primary: '#c0ffee',
      secondary: '#facade',
    },
  },
  plugins: [
    function ({ addUtilities }) {
      addUtilities({
        '.example': {
          color: 'red',
        },
      })
    },
  ],
}
