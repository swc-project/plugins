import styled from "styled-components";

const MyStyledComponent = styled.div`
  max-height: calc(
    ${someVariable1} + ${someVariable2} + ${someVariable3}
  ); // This comment causes a parsing error
  color: red;
`;
