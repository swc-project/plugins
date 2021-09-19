/** @jsx jsx */
import { Component } from 'react'
import { jsx, css } from '@emotion/react'

class SomeComponent extends Component {
  render() {
    return (
      <div
        css={css`
          color: hotpink;
        `}
      />
    )
  }
}
