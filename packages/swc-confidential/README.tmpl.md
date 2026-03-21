# @swc/plugin-swc-confidential

`@swc/plugin-swc-confidential` encrypts string literals marked with the `/*#__CONFIDENTIAL__*/` comment using AES encryption at build time. This lets you ship sensitive strings (such as feature flag names or identifiers) in an obfuscated form so they are not trivially readable in your production bundle.

## Usage

.swcrc:

```json
{
  "jsc": {
    "experimental": {
      "plugins": ["@swc/plugin-swc-confidential", {
        // Possible values:
        //
        // AES-128
        // AES-192
        // AES-256
        algorithm: "AES-256",
        encryptionKey: process.env.FLAGS_SECRET,
        prefix: "secure:"
      }]
    }
  }
}
```

### Input

```js
console.log(/*#__CONFIDENTIAL__*/ "newDashboard")
```

### Output



```js
console.log("secure:de454774988d624b8f317bbe9dadfe1f");
```