import styled from "styled-components";

const Box = ({ children, ...rest }) => <div {...rest}>{children}</div>;
const COLOR = "red";

export default function Home() {
  return <Box css={`color: ${COLOR}`}>Red color text</Box>;
}
