import fs from 'fs'
import path from 'path'
import postcss from 'postcss'
import prettier from 'prettier'

import tailwind from './index'

describe('plugins', () => {

  it('should generate CSS using external plugins defined in your tailwind.config.js file', () => {
    let input = readFile('./test-fixtures/external-plugin/index.css')
    let config = file('./test-fixtures/external-plugin/tailwind.config.js')

    return run(input, config).then((result) => {
      expect(format(result.css)).toEqual(
        format(css`
          .plugin-utility {
            color: green;
          }
        `),
      )
    })
  })

  it('should generate CSS using built-in, CSS and external plugins', () => {
    let input = readFile('./test-fixtures/combined-plugins/index.css')
    let config = file('./test-fixtures/combined-plugins/tailwind.config.js')

    return run(input, config).then((result) => {
      expect(format(result.css)).toEqual(
        format(css`
          .built-in-utility {
            color: red;
          }
          .plugin-utility {
            color: green;
          }
          .css-utility {
            color: blue;
          }
        `),
      )
    })
  })
})

// ---

// Ignore below, just some helper functions
let css = String.raw

function file(filePath) {
  return path.resolve(__dirname, filePath)
}

function readFile(filePath) {
  return fs.readFileSync(file(filePath), 'utf8')
}

function run(input, config, plugin = tailwind) {
  let { currentTestName } = expect.getState()

  return postcss(plugin(config)).process(input, {
    from: `${path.resolve(__filename)}?test=${currentTestName}`,
  })
}

// Just for a bit nicer diffs in the tests, nothing related to Tailwind itself.
function format(styles) {
  return prettier.format(styles, { parser: 'css' })
}
