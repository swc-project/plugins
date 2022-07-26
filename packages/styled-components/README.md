### `@swc/plugin-styled-components`

#### Setup

```sh
npm install --save-dev @swc/plugin-styled-components @swc/core@1.2.215
```

> @swc/core@1.2.215 is required for now

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
