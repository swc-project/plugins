import * as emotionReact from "@emotion/react";

export const pulse = emotionReact.keyframes`
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
