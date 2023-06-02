### `@swc/plugin-fusion-gql`

#### Setup

```sh
npm install --save-dev @swc/plugin-fusion-gql @swc/core
```

Then update your `.swcrc` file like below:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@swc/plugin-fusion-gql",
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
