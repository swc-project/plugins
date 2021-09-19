import { css } from '@emotion/react'

function thing() { }

function doThing() {
  return css`
    display: ${thing()};
  `
}
