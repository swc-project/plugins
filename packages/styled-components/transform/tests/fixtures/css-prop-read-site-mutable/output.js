import styled from "styled-components";
const Box = styled.div``;
var _StyledBox = styled(Box)`color: ${(p)=>p.$_css}; background: ${(p)=>p.$_css2};`;
let color = "red";
let calls = 0;
export function Home() {
    return <_StyledBox $_css={color} $_css2={getColor()}/>;
}
function getColor() {
    calls++;
    return color;
}
color = "blue";
