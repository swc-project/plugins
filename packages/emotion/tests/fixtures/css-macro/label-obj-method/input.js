import { css } from '@emotion/react'

const obj = {
  FooBar() {
    return (
      <div
        css={css`
          background-color: hotpink;
        `}
      />
    )
  }
}
