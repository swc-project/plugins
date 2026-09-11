import React from "react";
import styled from "styled-components";
const Box = styled.div.withConfig({
    displayName: "code__Box",
    componentId: "sc-4fac80da-0"
})([
    ``
]);
var _StyledBox = styled(Box).withConfig({
    displayName: "code___StyledBox",
    componentId: "sc-4fac80da-1"
})([
    `color:`,
    ``
], getColor());
const App = ()=><_StyledBox>Red</_StyledBox>;
function getColor() {
    return "red";
}
export default App;
