import path from 'path'

module.exports = {
  content: [path.resolve(__dirname, './index.html')],
  plugins: [
    function ({ addUtilities }) {
      addUtilities({
        '.plugin-utility': {
          color: 'green',
        },
      })
    },
  ],
}
