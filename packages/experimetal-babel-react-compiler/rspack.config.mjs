import { defineConfig } from "@rspack/cli";
import NodePolyfillPlugin from 'node-polyfill-webpack-plugin';
export default defineConfig({
  entry: {
    main: "./src/index.ts",
  },
  output: {
    library: {
      type: 'module',
    },
    module: true,
  },
  devtool: false,
  optimization: {
    minimize: false,
  },
  plugins: [new NodePolyfillPlugin({
    additionalAliases: ['process'],
  })],
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: {
          loader: 'builtin:swc-loader',
          options: {
            jsc: {
              parser: {
                syntax: 'typescript',
              },
              target: 'es2019'
            },
          }
        }
      }
    ]
  }
});
