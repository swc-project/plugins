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
  describe('aggressive import', () => {
    describe('without "webpackChunkName"', () => {
      it('should support simple request', () => {
        const result = testPlugin(`
          loadable(props => import(\`./\${props.foo}\`))
        `)

        expect(result).toMatchSnapshot()
      })

      it('should support complex request', () => {
        const result = testPlugin(`
          loadable(props => import(\`./dir/\${props.foo}/test\`))
        `)

        expect(result).toMatchSnapshot()
      })

      it('should support destructuring', () => {
        const result = testPlugin(`
          loadable(({ foo }) => import(\`./dir/\${foo}/test\`))
        `)

        expect(result).toMatchSnapshot()
      })
    })
  })

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

    it('should transpile function expression', () => {
      const result = testPlugin(`
        const load = /* #__LOADABLE__ */ function () {
          return import('moment')
        }
      `)
      expect(result).toMatchSnapshot()
    })

    it('should remove only needed comments', () => {
      const result = testPlugin(`
        const load = /* #__LOADABLE__ */ /* IMPORTANT! */ () => import('moment')
      `)

      expect(result).toMatchSnapshot()
    })
  })
})
