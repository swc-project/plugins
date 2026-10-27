import _styled from "styled-components";
import React from "react";
import { css } from "styled-components";
export default function Example() {
    return <_StyledDiv $_css={someCss}>oops</_StyledDiv>;
}
const someCss = css([
    `color:red;`
]);
var _StyledDiv = _styled("div").withConfig({
    displayName: "code___StyledDiv",
    componentId: "sc-ba25a50c-0"
})([
    ``,
    ``
], (p)=>p.$_css);
