import styled from 'styled-components';
const Box = styled.div``;
var _StyledBox3 = styled(Box)`bottom: ${SIZES[KEY].bottom};`;
var _StyledBox2 = styled(Box)`bottom: ${(p)=>p.$_css2};`;
var _StyledBox = styled(Box)`bottom: ${(p)=>p.$_css};`;
const SIZES = {
    small: {
        bottom: '8px'
    },
    large: {
        bottom: '16px'
    }
};
const KEY = 'small';
export const Local = ({ size })=><_StyledBox $_css={SIZES[size].bottom}/>;
export const LocalCall = ({ size })=><_StyledBox2 $_css2={SIZES.get(size).bottom}/>;
export const TopLevel = ()=><_StyledBox3/>;
