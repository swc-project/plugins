import styled from '@emotion/styled'
import { css } from '@emotion/react'

const H1 = styled.h1`
  ${props =>
    css`
      color: red;
    `}
  /* dummy comment */
  ${props =>
    css`
      text-transform: uppercase;
    `}
`
