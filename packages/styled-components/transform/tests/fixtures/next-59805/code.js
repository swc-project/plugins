"use client";

import styled from "styled-components";

const ReproElement = styled.p`
  &::before {
    content: "\\a9";
  }
`;

export function ReproComponent(props) {
  return (
    <ReproElement>
      This should be preceded by a copyright symbol and <i>not</i> backslash,
      letter A, digit nine
    </ReproElement>
  );
}
