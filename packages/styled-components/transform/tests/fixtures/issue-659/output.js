import styled from "styled-components";
const Box = styled.div([
    ``
]);
var _StyledBox = styled(Box)([
    `bottom: `,
    `; margin-bottom: `,
    `; padding-bottom: `,
    `; top: `,
    `;`
], (p)=>p.$_css, (p)=>p.$_css2, (p)=>p.$_css3, (p)=>p.$_css4);
const SIZES = {
    small: {
        bottom: "8px",
        linked: {
            bottom: "4px"
        }
    },
    large: {
        bottom: "16px",
        linked: {
            bottom: "8px"
        }
    }
};
export const Avatar = ({ size, property })=><_StyledBox $_css={SIZES[size].bottom} $_css2={SIZES[size].linked.bottom} $_css3={SIZES[size].linked[property]} $_css4={SIZES.small.bottom}/>;
