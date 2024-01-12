// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/object-dynamic-property.js

import { css } from "@emotion/react";

function doThing() {
  return {
    [css({ color: "hotpink" })]: "coldblue",
  };
}
