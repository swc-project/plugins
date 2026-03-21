### `@swc/plugin-styled-components`

`@swc/plugin-styled-components` is the SWC equivalent of `babel-plugin-styled-components`. It adds display names to styled-components for better debugging, generates deterministic class names for server-side rendering (SSR), and enables other optimizations provided by the styled-components Babel plugin.

#### Setup

```sh
npm install --save-dev @swc/plugin-styled-components @swc/core
```

Then update your `.swcrc` file like below:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@swc/plugin-styled-components",
          {
            "displayName": true,
            "ssr": true
          }
        ]
      ]
    }
  }
}
```

${CHANGELOG}
