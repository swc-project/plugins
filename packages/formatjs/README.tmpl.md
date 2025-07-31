# @swc/plugin-formatjs

FormatJS SWC plugin, maintained by SWC team.

## Usage

The default message descriptors for the app's default language will be processed from: `defineMessages()`, `defineMessage()`, `intl.formatMessage` and `<FormattedMessage>`; all of which are named exports of the React Intl package.

### Next.js

```tsx
import { NextConfig } from 'next';

const nextConfig: NextConfig = {
  experimental: {
    swcPlugins: [
      [
        require.resolve('@swc/plugin-formatjs'),
        {
          idInterpolationPattern: '[md5:contenthash:hex:10]',
          additionalComponentNames: ['F'],
          ast: true,
        },
      ],
    ],
  }
}
```

### Vite
```tsx
import react from '@vitejs/plugin-react-swc'
import { defineConfig } from 'vite'

export default defineConfig((env) => ({
  plugins: [
    react({
      tsDecorators: true,
      plugins: [
        [
          '@swc/plugin-formatjs',
          {
            idInterpolationPattern: '[md5:contenthash:hex:10]',
            additionalComponentNames: ['F'],
            ast: true,
          },
        ],
      ],
    }),
  ]
}))
```

## Options

### **`idInterpolationPattern`**

A template string that allows you to override the ID both in the extracted javascript and messages. It's used only if the message ID is empty.

### **`removeDefaultMessage`**

Remove `defaultMessage` field in generated js after extraction.

### **`extractSourceLocation`**

Whether the metadata about the location of the message in the source file should be extracted. If `true`, then `file`, `start`, and `end` fields will exist for each extracted message descriptors. Defaults to `false`.

### **`additionalComponentNames`**

Additional component names to extract messages from, e.g: `['FormattedFooBarMessage']`. **NOTE**: By default we check for the fact that `FormattedMessage` are imported from `moduleSourceName` to make sure variable alias works. This option does not do that so it's less safe.

### **`additionalFunctionNames`**

Additional function names to extract messages from, e.g: `['$formatMessage']`. Use this if you prefer to alias `formatMessage` to something shorter like `$t`.

### **`ast`**

Pre-parse `defaultMessage` into AST for faster runtime perf. This flag doesn't do anything when `removeDefaultMessage` is `true`.

### **`preserveWhitespace`**

Whether to preserve whitespace and newlines.

### **`pragma`**

A string that allows you to extract metadata from comments in your source files. When set, the plugin will look for comments containing the pragma string and parse key:value pairs that follow it.

For example, with `pragma: "@formatjs"`, a comment like:

```js
// @formatjs project:web locale:en region:us
```

Will extract the metadata: `{project: "web", locale: "en", region: "us"}` that gets included with the extracted messages. This is useful for adding contextual information to your message extractions.

${CHANGELOG}
