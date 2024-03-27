// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/actual-expected-usage.js

import { jsx, css } from "@emotion/react";

const SomeComponent = () => (
  <div
    css={css`
      color: hotpink;
    `}
  />
);
