/* eslint-disable import/no-extraneous-dependencies */
import { transform } from '@babel/core'
import plugin from '.'

const testPlugin = code => {
  const result = transform(code, {
    plugins: [plugin],
    configFile: false,
  })

  return result.code
}

describe('plugin', () => {
  describe('Magic comment', () => {
    it('should transpile shortand properties', () => {
      const result = testPlugin(`
        const obj = {
          /* #__LOADABLE__ */
          load() {
            return import('moment')
          }
        }
      `)

      expect(result).toMatchSnapshot()
    })

    it('should transpile arrow functions', () => {
      const result = testPlugin(`
        const load = /* #__LOADABLE__ */ () => import('moment')
      `)

      expect(result).toMatchSnapshot()
    })

  })
})
