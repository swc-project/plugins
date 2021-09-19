import { keyframes } from '@emotion/css'

let endingRotation = window.whatever

const rotate360 = keyframes`
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(${endingRotation});
  }
`
