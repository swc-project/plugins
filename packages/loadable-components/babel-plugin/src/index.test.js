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
    it('should work with destructuration', () => {
      const result = testPlugin(`
        loadable(({ foo }) => import(/* webpackChunkName: "Pages" */ \`./\${foo}\`))
      `)
      expect(result).toMatchSnapshot()
    })

    describe('with "webpackChunkName"', () => {
      it('should replace it', () => {
        const result = testPlugin(`
          loadable(props => import(/* webpackChunkName: "Pages" */ \`./\${props.foo}\`))
        `)

        expect(result).toMatchSnapshot()
      })

      it('should keep it', () => {
        const result = testPlugin(`
          loadable(props => import(/* webpackChunkName: "pages/[request]" */ \`./pages/\${props.path}\`))
        `)

        expect(result).toMatchSnapshot()
        expect(result).toEqual(
          expect.stringContaining('return "pages/" + props.path.replace'),
        )
        expect(result).toEqual(
          expect.stringContaining('/* webpackChunkName: "pages/[request]"'),
        )
      })
    })

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
