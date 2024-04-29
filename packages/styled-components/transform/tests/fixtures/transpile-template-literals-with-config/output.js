import styled, { css, createGlobalStyle } from "styled-components";
const Named = styled.div.withConfig({
    displayName: "code__Named"
})([
    `
  width: 100%;
`
]);
const NamedWithInterpolation = styled.div.withConfig({
    displayName: "code__NamedWithInterpolation"
})([
    `
  color: `,
    `;
`
], (color)=>props.color);
const Wrapped = styled(Inner).withConfig({
    displayName: "code__Wrapped"
})([
    `
  color: red;
`
]);
const Foo = styled.div.withConfig({
    displayName: "code__Foo"
})({
    color: "green"
});
const style = css([
    `
  background: green;
`
]);
const GlobalStyle = createGlobalStyle([
    `
  html {
    background: silver;
  }
`
]);
