import styled from "@emotion/styled";

export default function IndexPage() {
  return (
    <>
      <h1>IndexPage</h1>
      <IconWrapper className={"icon-chat"} />
      <IconWrapper className={"icon-check"} />
    </>
  );
}

const IconWrapper = styled.div`
  &[class^="icon-"],
  [class*=" icon-"] {
    color: red;
  }

  &.icon-chat:before {
    content: "\\e904";
  }

  &.icon-check:before {
    content: "\\e905";
  }
`;
