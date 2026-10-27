import styled from "styled-components";

const Box = styled.div``;
export const node = <Box css={`color: ${getColor()}; background: ${color};`} />;

function getColor() {
  return "red";
}

import { color } from "./colors";
