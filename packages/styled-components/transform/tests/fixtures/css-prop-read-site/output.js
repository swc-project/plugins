import styled from "styled-components";
const Box = ({ children, ...rest })=><div {...rest}>{children}</div>;
var _StyledBox = styled(Box)`color: ${(p)=>p.$_css}`;
const COLOR = "red";
export default function Home() {
    return <_StyledBox $_css={COLOR}>Red color text</_StyledBox>;
}
