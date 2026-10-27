import styled from "styled-components";
const Box = styled.div``;
var _StyledBox = styled(Box)`
    bottom: ${(p)=>p.$_css};
    bottom: ${(p)=>p.$_css2};
    bottom: ${(p)=>p.$_css3};
    bottom: ${(p)=>p.$_css4};
    bottom: ${(p)=>p.$_css5};
    bottom: ${(p)=>p.$_css6};
  `;
const SIZES = {
    small: {
        linked: {
            bottom: "8px"
        }
    },
    large: {
        linked: {
            bottom: "16px"
        }
    }
};
const getSizes = (size)=>SIZES[size];
const key = "small";
export const Avatar = ({ size, property })=><_StyledBox $_css={SIZES[size].linked.bottom} $_css2={SIZES[size].linked[property]} $_css3={SIZES[key].linked[property]} $_css4={getSizes(size).linked.bottom} $_css5={SIZES[size].getBottom()} $_css6={SIZES[getSizes(size).key].linked.bottom}/>;
export const StaticAvatar = ()=><_StyledDiv/>;
var _StyledDiv = styled("div")`
    bottom: ${SIZES.small.linked.bottom};
    bottom: ${SIZES["small"].linked.bottom};
    bottom: ${SIZES[key].linked.bottom};
    bottom: ${getSizes("small").linked.bottom};
  `;
