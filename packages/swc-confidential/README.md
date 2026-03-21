# @swc/plugin-swc-confidential

Encrypts confidential string literals at build time. Strings marked with the `/*#__CONFIDENTIAL__*/` comment annotation are replaced with AES-encrypted values, preventing sensitive data (such as feature flag names, API keys, or internal identifiers) from being exposed as plain text in the compiled bundle.

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