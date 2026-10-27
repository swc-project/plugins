import styled from "styled-components";
const Box = styled.div``;
var _StyledBox = styled(Box)`color: ${(p)=>p.$_css}; background: ${(p)=>p.$_css2};`;
export const node = <_StyledBox $_css={getColor()} $_css2={color}/>;
function getColor() {
    return "red";
}
import { color } from "./colors";
