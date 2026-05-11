import styled from "styled-components";
const someVariable1 = "10px";
const someVariable2 = "20px";
const someVariable3 = "30px";
const MyStyledComponent = styled.div([
    `max-height:calc( `,
    ` + `,
    ` + `,
    ` );`
], someVariable1, someVariable2, someVariable3);
