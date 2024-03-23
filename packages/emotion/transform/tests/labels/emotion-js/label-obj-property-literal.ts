// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/label-obj-property-literal.js

import { css } from "@emotion/react";

const obj = {
  "red component": (
    <div
      css={css`
        background-color: hotpink;
      `}
    />
  ),
};
