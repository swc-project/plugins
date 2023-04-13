# transform-imports

## Config

```json
[
  "transform-imports",
  {
    "react-bootstrap": {
      "transform": "react-bootstrap/lib/{{member}}"
    },
    "lodash": {
      "transform": "lodash/{{member}}"
    },
    "antd": {
      "transform": "antd/es/{{kebabCase member}}",
      "sideEffect": "antd/es/{{kebabCase member}}/style"
    }
  }
]
```
