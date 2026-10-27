import styled, { css } from "styled-components";
const Box = styled.div``;
var _StyledBox = styled(Box)`color: ${(p)=>p.$_css}; background: ${(p)=>p.$_css2}; border-color: ${(p)=>p.$_css3};`;
export function Home() {
    return <_StyledBox $_css={palette[current]()} $_css2={getColor()} $_css3={palette.red}/>;
}
const palette = {
    red: "red",
    getRed: ()=>"red"
};
const current = "getRed";
const getColor = ()=>"blue";
