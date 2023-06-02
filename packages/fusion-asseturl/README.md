### `@swc/plugin-fusion-asseturl`

#### Setup

```sh
npm install --save-dev @swc/plugin-fusion-asseturl @swc/core
```

Then update your `.swcrc` file like below:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@swc/plugin-fusion-asseturl",
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
