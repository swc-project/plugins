import { defineConfig } from "@rspack/cli";
import nodePolyfillPlugin from 'node-polyfill-webpack-plugin';
export default defineConfig({
  entry: {
    main: "./src/index.ts",
  },
  output: {
    library: {
      type: 'module',
      export: ['transform']
    }
  },
  optimization: {
    minimize: false,
  },
  plugins: [new nodePolyfillPlugin()],
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
            },
          }
        }
      }
    ]
  }
});
