import { styled } from "styled-components";

const Div = styled.div`
  &::after {
    content: "Hello\u0020World!\u{1f64f}";
  }
`;