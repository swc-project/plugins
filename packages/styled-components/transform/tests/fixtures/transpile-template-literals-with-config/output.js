import styled, { css, createGlobalStyle } from 'styled-components';
const Named = styled.div.withConfig({
    displayName: "code__Named"
})([
    "\n  width: 100%;\n"
]);
const NamedWithInterpolation = styled.div.withConfig({
    displayName: "code__NamedWithInterpolation"
})([
    "\n  color: ",
    ";\n"
], (color)=>props.color);
const Wrapped = styled(Inner).withConfig({
    displayName: "code__Wrapped"
})([
    "\n  color: red;\n"
]);
const Foo = styled.div.withConfig({
    displayName: "code__Foo"
})({
    color: 'green'
});
const style = css([
    "\n  background: green;\n"
]);
const GlobalStyle = createGlobalStyle([
    "\n  html {\n    background: silver;\n  }\n"
]);
