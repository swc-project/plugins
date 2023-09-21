# react-remove-properties

See https://nextjs.org/docs/architecture/nextjs-compiler#remove-react-properties for more information.

## Config

```json
["react-remove-properties"]
```

or

```json
[
  "react-remove-properties",
  {
    // The regexes defined here are processed in Rust so the syntax is different from
    // JavaScript `RegExp`s. See https://docs.rs/regex.
    "properties": ["^data-custom$"]
  }
]
```
