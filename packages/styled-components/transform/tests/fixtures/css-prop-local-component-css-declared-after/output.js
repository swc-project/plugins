import React from "react";
import styled from "styled-components";
const Box = styled.div.withConfig({
    displayName: "code__Box",
    componentId: "sc-82ab8374-0"
})([
    ``
]);
const COLOR = "red";
var _StyledBox = styled(Box).withConfig({
    displayName: "code___StyledBox",
    componentId: "sc-82ab8374-1"
})([
    `color:`,
    ``
], COLOR);
export default function Example() {
    return <_StyledBox>Red</_StyledBox>;
}
