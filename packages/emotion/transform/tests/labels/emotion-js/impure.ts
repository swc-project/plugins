// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/impure.js

import { css } from "@emotion/react";

function thing() {}

function doThing() {
  return css`
    display: ${thing()};
  `;
}
