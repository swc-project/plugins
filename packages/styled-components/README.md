### `@swc/plugin-styled-components`

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
