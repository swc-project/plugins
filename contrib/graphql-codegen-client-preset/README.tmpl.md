# `@swc-contrib/plugin-graphql-codegen-client-preset`

When using the [`@graphql-codegen/client-preset`](https://the-guild.dev/graphql/codegen/plugins/presets/preset-client) on large scale projects might want to enable code splitting or tree shaking on the `client-preset` generated files. This is because instead of using the map which contains all GraphQL operations in the project, we can use the specific generated document types.

This plugin works for [SWC](https://swc.rs) only.

### Installation

```bash
yarn add -D @swc-contrib/plugin-graphql-codegen-client-preset
```

### Usage

You will need to provide the `artifactDirectory` path that should be the same as the one configured in your `codegen.ts`

The plugin also supports a `namingConvention` option to match the naming convention configured in your `codegen.ts`. The default is `"change-case-all#pascalCase"` which matches the default for `@graphql-codegen/client-preset`. If you have set `namingConvention: "change-case-all#upperCaseFirst"` in your `codegen.ts`, you must also set it in the plugin options.

The following options must also match your Codegen configuration when you change document variable names:

| Option | Default | Effect |
| --- | --- | --- |
| `documentVariablePrefix` | `""` | Prefix for query, mutation, and subscription document variables. |
| `documentVariableSuffix` | `"Document"` | Suffix for query, mutation, and subscription document variables. Use `""` to remove it. |
| `fragmentVariablePrefix` | `""` | Prefix for fragment document variables. |
| `fragmentVariableSuffix` | `"FragmentDoc"` | Suffix for fragment document variables. Use `""` to remove it. |
| `dedupeOperationSuffix` | `false` | Remove the leading `Fragment` from the fragment suffix when the fragment name already ends with `Fragment`. The comparison ignores case. |
| `omitOperationSuffix` | `false` | Omit the fragment variable suffix. This takes precedence over `dedupeOperationSuffix`. |

As in Codegen, `dedupeOperationSuffix` and `omitOperationSuffix` do not change operation document variable names. The plugin applies `namingConvention` after combining the prefix, name, and suffix. The supported string naming conventions preserve underscores and convert each underscore-separated segment independently.

For example, if Codegen uses `documentVariableSuffix: ""`, set the same option in the plugin:

```json
{
  "artifactDirectory": "./src/gql",
  "gqlTagName": "graphql",
  "documentVariableSuffix": ""
}
```

A call for `query GetUser { id }` then imports `GetUser` instead of `GetUserDocument`.

#### Vite

```ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react({
      plugins: [
        [
          "@swc-contrib/plugin-graphql-codegen-client-preset",
          { artifactDirectory: "./src/gql", gqlTagName: "graphql" },
        ],
      ],
    }),
  ],
});
```

#### Next.js

```ts
const nextConfig = {
  // ...
  experimental: {
    swcPlugins: [
      [
        "@swc-contrib/plugin-graphql-codegen-client-preset",
        { artifactDirectory: "./src/gql", gqlTagName: "graphql" },
      ],
    ],
  },
};
```

#### `.swcrc`

```json5
{
  // ...
  jsc: {
    // ...
    experimental: {
      plugins: [
        [
          "@swc-contrib/plugin-graphql-codegen-client-preset",
          { artifactDirectory: "./src/gql", gqlTagName: "graphql" },
        ],
      ],
    },
  },
}
```

${CHANGELOG}
