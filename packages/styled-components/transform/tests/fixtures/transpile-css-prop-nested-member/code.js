import styled from 'styled-components';

const Box = styled.div``;
const SIZES = { small: { bottom: '8px' }, large: { bottom: '16px' } };
const KEY = 'small';

export const Local = ({ size }) => <Box css={`bottom: ${SIZES[size].bottom};`} />;

export const LocalCall = ({ size }) => <Box css={`bottom: ${SIZES.get(size).bottom};`} />;

export const TopLevel = () => <Box css={`bottom: ${SIZES[KEY].bottom};`} />;
