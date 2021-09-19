import { css } from '@emotion/react'

const thing = css`
  display: flex;
  &:hover {
    ${css`
      color: hotpink;
    `};
  }
`
