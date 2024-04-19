# @swc/plugin-swc-confidential

## Usage

.swcrc:

```json
{
  "jsc": {
    "experimental": {
      "plugins": ["@swc/plugin-swc-confidential", {
        algorithm: "",
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