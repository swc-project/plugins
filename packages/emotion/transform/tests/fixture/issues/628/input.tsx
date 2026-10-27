import { css } from "@emotion/react";

const path = "https://cdn.example.com/fonts/";
const family = "MyFont";

export const styles = css`
  @font-face {
    font-family: '${family}';
    src: url('${path}${family}-Bold.woff2') format('woff2');
  }
`;
