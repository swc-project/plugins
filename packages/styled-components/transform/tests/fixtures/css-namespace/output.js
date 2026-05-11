import styled, { css, createGlobalStyle } from "styled-components";
const Button = styled.button([
    `.myapp &{color:red;&:hover{color:blue;}}`
]);
const Wrapped = styled(Link)([
    `.myapp &{color:green;}`
]);
const ObjectStyle = styled.div({
    color: "black"
});
const helper = css([
    `color:hotpink;`
]);
const GlobalStyle = createGlobalStyle([
    `body{margin:0;}`
]);
