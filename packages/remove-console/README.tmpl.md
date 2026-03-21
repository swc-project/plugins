# remove-console

`@swc/plugin-remove-console` strips `console.*` calls from your production builds, helping to reduce bundle size and avoid leaking debug information to end users. You can optionally exclude specific console methods (e.g. `console.error`) from removal. See https://nextjs.org/docs/architecture/nextjs-compiler#remove-console for more information.

## Config

```json
["@swc/plugin-remove-console"]
```

or

```json
[
  "@swc/plugin-remove-console",
  {
    "exclude": ["error"]
  }
]
```

${CHANGELOG}
