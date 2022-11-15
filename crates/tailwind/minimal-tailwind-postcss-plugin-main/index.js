import { tailwind } from './tailwind.js'

// PostCSS Plugin
export default function tailwindcss(configPath) {
  return {
    postcssPlugin: 'tailwindcss',
    plugins: [
      function (root, result) {
        tailwind({ root, result, configPath })
      },
    ],
  }
}

tailwindcss.postcss = true
