"use client";
import styled from "styled-components";
const ReproElement = styled.p.withConfig({
    displayName: "code__ReproElement",
    componentId: "sc-d5fbeba3-0"
})([
    `&::before{content:"\\a9";}`
]);
export function ReproComponent(props) {
    return <ReproElement>
      This should be preceded by a copyright symbol and <i>not</i> backslash,
      letter A, digit nine
    </ReproElement>;
}
