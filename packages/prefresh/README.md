# @swc/plugin-prefresh

The SWC implementation of the [prefresh babel plugin](https://github.com/preactjs/prefresh/tree/main/packages/babel).

## Usage

Prefresh babel plugin is a forked equivalent of the react-refresh babel plugin difference being that we need a way to memoize createContext between HMR.

And SWC has built-in React Refresh transformation, therefore, this plugin only implements the `createContext` processing part and need to be used with `jsc.transform.react.refresh`.

.swcrc:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "@swc/plugin-prefresh", // enable prefresh specific transformation
          {
            "library": ["preact-like-framework"] // the customizable preact name, default is `["preact", "preact/compat", "react"]`
          }
        ]
      ]
    },
    "parser": {
      "jsx": true
    },
    "transform": {
      "react": {
        "development": true,
        "refresh": true, // enable react refresh transformation
      }
    }
  }
}
```
