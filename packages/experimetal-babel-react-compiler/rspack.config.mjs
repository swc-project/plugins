import { defineConfig } from "@rspack/cli";
import nodePolyfillPlugin from 'node-polyfill-webpack-plugin';
export default defineConfig({
  entry: {
    main: "./src/index.ts",
  },
  output: {
    library: {
      type: 'var',
      name: 'transform'
    }
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
