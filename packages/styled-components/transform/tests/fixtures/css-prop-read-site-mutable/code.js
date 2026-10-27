import styled from "styled-components";

const Box = styled.div``;
let color = "red";
let calls = 0;

export function Home() {
  return <Box css={`color: ${color}; background: ${getColor()};`} />;
}

function getColor() {
  calls++;
  return color;
}

color = "blue";
