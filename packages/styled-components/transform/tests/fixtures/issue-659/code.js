import styled from "styled-components";

const Box = styled.div``;
const SIZES = {
  small: { bottom: "8px", linked: { bottom: "4px" } },
  large: { bottom: "16px", linked: { bottom: "8px" } },
};

export const Avatar = ({ size, property }) => (
  <Box css={`bottom: ${SIZES[size].bottom}; margin-bottom: ${SIZES[size].linked.bottom}; padding-bottom: ${SIZES[size].linked[property]}; top: ${SIZES.small.bottom};`} />
);
