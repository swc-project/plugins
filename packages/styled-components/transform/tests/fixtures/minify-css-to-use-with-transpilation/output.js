import styled from "styled-components";
const Simple = styled.div([
    `width:100%;`
]);
const Interpolation = styled.div([
    `content:"  `,
    `  ";`
], (props)=>props.text);
const SpecialCharacters = styled.div([
    `content:"  `,
    `  ";color:red;`
], (props)=>props.text);
const Comment = styled.div([
    `color:red;`
]);
const Parens = styled.div([
    `&:hover{color:blue;}`
]);
