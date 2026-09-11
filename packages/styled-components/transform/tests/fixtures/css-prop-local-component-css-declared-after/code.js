import React from "react";
import styled from "styled-components";

const Box = styled.div``;

const COLOR = "red";

export default function Example() {
  return <Box css={`color: ${COLOR}`}>Red</Box>;
}
