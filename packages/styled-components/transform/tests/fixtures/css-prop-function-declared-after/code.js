import React from "react";
import styled from "styled-components";

const Box = styled.div``;

const App = () => (
  <Box css={`color: ${getColor()}`}>Red</Box>
);

function getColor() {
  return "red";
}

export default App;
