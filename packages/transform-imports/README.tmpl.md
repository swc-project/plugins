# transform-imports

`@swc/plugin-transform-imports` rewrites named imports into direct, member-level imports to reduce bundle size. For example, `import { Button } from 'react-bootstrap'` becomes `import Button from 'react-bootstrap/lib/Button'`, which allows bundlers to include only the specific module rather than the entire library.

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
    }
  }
]
```

${CHANGELOG}
