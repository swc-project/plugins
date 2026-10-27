import { css, keyframes } from "@emotion/react";

const pulse = keyframes`
  0% {
    opacity: 1;
  }

  50% {
    opacity: 0.5;
  }

  100% {
    opacity: 1;
  }
`;

const className = css`
  animation: ${pulse} 1s infinite;
`;
