### `@swc/plugin-fusion`

#### Setup

```sh
npm install --save-dev @swc/plugin-fusion @swc/core
```

Then update your `.swcrc` file like below:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@swc/plugin-fusion",
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
