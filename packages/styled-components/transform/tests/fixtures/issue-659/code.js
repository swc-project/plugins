import styled from "styled-components";

const Box = styled.div``;
const SIZES = {
  small: { linked: { bottom: "8px" } },
  large: { linked: { bottom: "16px" } },
};
const getSizes = (size) => SIZES[size];
const key = "small";

export const Avatar = ({ size, property }) => (
  <Box css={`
    bottom: ${SIZES[size].linked.bottom};
    bottom: ${SIZES[size].linked[property]};
    bottom: ${SIZES[key].linked[property]};
    bottom: ${getSizes(size).linked.bottom};
    bottom: ${SIZES[size].getBottom()};
    bottom: ${SIZES[getSizes(size).key].linked.bottom};
  `} />
);

export const StaticAvatar = () => (
  <div css={`
    bottom: ${SIZES.small.linked.bottom};
    bottom: ${SIZES["small"].linked.bottom};
    bottom: ${SIZES[key].linked.bottom};
    bottom: ${getSizes("small").linked.bottom};
  `} />
);
