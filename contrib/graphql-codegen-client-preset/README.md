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
