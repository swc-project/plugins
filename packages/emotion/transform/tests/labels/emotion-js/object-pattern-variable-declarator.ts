// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/object-pattern-variable-declarator.js

import { css } from "@emotion/react";
import { extractCritical } from "@emotion/server";
import React from "react";
import { renderToString } from "react-dom/server";

const { css: styles } = extractCritical(
  renderToString(
    <div
      css={css`
        color: hotpink;
      `}
    />
  )
);
