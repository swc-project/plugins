import styled, { css } from "styled-components";

const Box = styled.div``;

export function Home() {
  return <Box css={css`color: ${palette[current]()}; background: ${getColor()}; border-color: ${palette.red};`} />;
}

const palette = { red: "red", getRed: () => "red" };
const current = "getRed";
const getColor = () => "blue";
