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

Additional function names to extract messages from, e.g: `['']`. Use this if you prefer to alias `formatMessage` to something shorter like ``.

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

# @swc/plugin-formatjs

## 6.0.3

### Patch Changes

- 08db5e1: docs(formatjs): add Usage to readme
- beabfa1: Add proper README

## 6.0.2

### Patch Changes

- 27cae40: Disallow typescript types to shrink size

## 6.0.1

### Patch Changes

- df280ad: Update swc_core to v34.0.0

## 6.0.0

### Major Changes

- 562e755: Update swc_core to v33

## 5.1.4

### Patch Changes

- 9a27285: fix(formatjs): Fix description extract for the template literal string

## 5.1.3

### Patch Changes

- 85719ca: Update swc_core to v32

## 5.1.2

### Patch Changes

- 467573b: fix(formatjs): Fix ast transform in jsx

## 5.1.1

### Patch Changes

- 45d1ac7: Update swc_core to v31

## 5.1.0

### Minor Changes

- 899d803: Add support for md5 hash_type in idInterpolationPattern

## 5.0.0

### Major Changes

- f7c8ad4: swc_core v29

## 4.0.1

### Patch Changes

- 8d5ce5c: Update swc_core to v28.0.0

## 4.0.0

### Major Changes

- cf2636b: Update swc_core to v27

## 3.2.4

### Patch Changes

- e3e743d: Update swc_core to v27

## 3.2.3

### Patch Changes

- f226c4b: Generate the same ID after React Compiler

## 3.2.2

### Patch Changes

- 5ddbaeb: Update swc_core to v23

## 3.2.1

### Patch Changes

- 1abaa28: fix(formatjs): Sort description objects before serialization

## 3.2.0

### Minor Changes

- 648faf7: add support for hex and base64url digest encodings in idInterpolationPattern placeholders

## 3.1.2

### Patch Changes

- d51d525: Update swc_core to v22.0.0

## 3.1.1

### Patch Changes

- cb94b92: Update swc_core to v21.0.1

## 3.1.0

### Minor Changes

- af2e35d: Add support for sha1 hash_type in idInterpolationPattern

## 3.0.1

### Patch Changes

- 31e3254: build: Update `swc_core` to `v19.0.0`

## 3.0.0

### Major Changes

- f0fee1d: Update swc_core to v15.0.1

## 2.4.0

### Minor Changes

- 72385e1: add support for idInterpolationPattern placeholders

## 2.3.1

### Patch Changes

- 04465bc: Update swc_core to v14.0.0, really

## 2.3.0

### Minor Changes

- bfa0a51: Update swc_core to v13

## 2.2.0

### Minor Changes

- b8c4e6c: Update swc_core to v12

## 2.1.0

### Minor Changes

- 4c8b0e2: Update swc_core

## 2.0.6

### Patch Changes

- e8973e8: Update swc_core to v10.2.3

## 2.0.5

### Patch Changes

- f436a09: Update swc_core to v10.

## 2.0.4

### Patch Changes

- 2ae5319: Fix parsing of unicode

## 2.0.3

### Patch Changes

- bb13cc8: Fix missing quotes of plural keys

## 2.0.2

### Patch Changes

- f155bce: Update swc_core to v9

## 2.0.1

### Patch Changes

- c9e75ce: Bump crate versions

## 2.0.0

### Major Changes

- 4574a70: Update swc_core to v8.0.1

## 1.0.3

### Patch Changes

- f3cea5f: Bump versions

## 1.0.2

### Patch Changes

- a73664c: Update swc_core to v6.0.2

## 1.0.1

### Patch Changes

- 4ff3b22: Move formatjs plugin to official plugin repository
