import fs from 'fs'
import path from 'path'

import postcss from 'postcss'
import postcssJs from 'postcss-js'

import fg from 'fast-glob'
import globParent from 'glob-parent'

// Tailwind Core
export function tailwind({ root, result, configPath }) {
  // Add dependency message for watching related purposes so that we can rebuild the css if any of
  // the `content` files (and globs) change.
  function watchFile(file) {
    result.messages.push({
      plugin: 'tailwindcss',
      parent: result.opts.from,
      type: 'dependency',
      file,
    })
  }

  function watchDir(dir, glob) {
    result.messages.push({
      plugin: 'tailwindcss',
      parent: result.opts.from,
      type: 'dir-dependency',
      dir,
      glob, // Webpack ignores this, but other tools like Vite use it when present.
    })
  }

  // Ensure that requiring the config file does not load a cached version of the module.
  delete require.cache[configPath]

  // Ensure we watch the config file as a dependency so we can rebuild when it changes.
  watchFile(configPath)

  // Resolve the config file
  let config = require(configPath)

  // We use fast-glob for resolving all the globs defined in your `content` section. Ideally we can
  // register the glob itself as a dependency so that <build-tool> can watch it and ensure that when
  // you add new files we can rebuild the CSS.
  let files = fg.sync(config.content)

  // Register the dir/globs as dependencies.
  for (let pattern of config.content) {
    let { base, glob } = parseGlob(pattern)
    watchDir(base, glob)
  }

  // Read each template file and extract the classes that are being used.
  let candidates = new Set()

  // Ideally we don't need to read all of these files on subsequent builds in watch mode, and
  // the build tool will inform us which files have changed so we can be more efficient and
  // just read those files. In Tailwind today we do a lot of work to track this ourselves
  // and avoid unnecessary work because build tools don't provide it to us.
  for (let file of files) {
    let contents = fs.readFileSync(path.resolve(__dirname, file), 'utf8')
    // NOTE: Details here are not important, most important part is that we can "read" the files.
    // In reality this is a lot more complicated but it doesn't matter for this proof-of-concept.
    for (let candidate of contents.split(/['"\s<>=/]/g)) {
      candidates.add(candidate)
    }
  }

  // Tailwind's core is a big list of "plugins" that generate CSS.
  // Those plugins can come from three places:
  // - Baked in to the core of Tailwind itself
  // - User-authored CSS defined within a `@layer` is parsed and
  //   converted into a plugin by Tailwind at build time
  // - User-authored plugins can be written in JS and registered
  //   in the `tailwind.config.js` file
  let plugins = []

  // Built-in plugins
  plugins.push(function ({ addUtilities }) {
    addUtilities({
      '.built-in-utility': {
        color: 'red',
      },
      '.should-not-be-generated': {
        appearance: 'none',
      },
      // Etc.
    })
  })

  // Example built-in plugin that can read values from the config.
  plugins.push(function ({ addUtilities }) {
    addUtilities(
      Object.fromEntries(
        Object.entries(config?.theme?.colors ?? {}).map(([name, value]) => [`.text-${name}`, { color: value }]),
      ),
    )
  })

  // External plugins registered in the `tailwind.config.js` file.
  if (config.plugins) {
    plugins = plugins.concat(config.plugins)
  }

  // Collect "plugins" from the CSS
  //
  // NOTE: In reality we want to collect information for the correct layer. But for this proof of
  // concept that does not matter. Idea is that we _can_ read the CSS file and collect information
  // from it.
  root.walkAtRules('layer', (layer) => {
    layer.walkRules((node) => {
      let declarations = {}
      node.walkDecls((decl) => {
        declarations[decl.prop] = decl.value
      })

      plugins.push(function ({ addUtilities }) {
        addUtilities({
          [node.selector]: declarations,
        })
      })
    })

    // Remove the layer from the CSS.
    layer.remove()
  })

  // Generate all of the CSS by looking at the classes extracted from the template files
  // registered in the user's `content` configuration and matching them with the plugins
  // we registered with Tailwind above.
  let newRules = []
  for (let plugin of plugins) {
    plugin({
      addUtilities(definition) {
        for (let [selector, declarations] of Object.entries(definition)) {
          // Only generate the rules that we care about.
          // .slice(1) is a quick way of getting rid of the `.` of the selector
          // Very naive, but as a proof-of-concept this is fine.
          if (candidates.has(selector.slice(1))) {
            for (let node of parseObjectStyles({ [selector]: declarations })) {
              newRules.push(node)
            }
          }
        }
      },
    })
  }

  // Replace the @tailwind rule with the CSS that was generated based on the user's
  // template contents.
  root.walkAtRules('tailwind', (node) => {
    node.replaceWith(newRules)
    node.remove()
  })
}

// A function that allows us to generate PostCSS nodes from raw objects.
function parseObjectStyles(styles) {
  if (!Array.isArray(styles)) {
    return parseObjectStyles([styles])
  }

  return styles.flatMap((style) => {
    return postcss().process(style, {
      parser: postcssJs,
    }).root.nodes
  })
}

// Based on `glob-base`
// https://github.com/micromatch/glob-base/blob/master/index.js
export function parseGlob(pattern) {
  let glob = pattern
  let base = globParent(pattern)

  if (base !== '.') {
    glob = pattern.substr(base.length)
    if (glob.charAt(0) === '/') {
      glob = glob.substr(1)
    }
  }

  if (glob.substr(0, 2) === './') {
    glob = glob.substr(2)
  }
  if (glob.charAt(0) === '/') {
    glob = glob.substr(1)
  }

  return { base, glob }
}
