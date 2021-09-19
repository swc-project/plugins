import { injectGlobal } from '@emotion/css'

injectGlobal`
  body {
    margin: 0;
    padding: 0;
    & > div {
      display: flex;
    }
  }
  html {
    background: green;
  }
`
