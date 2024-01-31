import styled from "@emotion/styled";

export default function IndexPage() {
    return (
        <>
            <h1>IndexPage</h1>
            <IconWrapper className={'icon-chat'} />
            <IconWrapper className={'icon-check'} />
        </>
    );
}

const IconWrapper = styled.div`
  &[class^="icon-"], [class*=" icon-"] {
    /* use !important to prevent issues with browser extensions that change fonts */
    font-family: 'icomoon' !important;
    speak: never;
    font-style: normal;
    font-weight: 400;
    font-variant: normal;
    text-transform: none;
    font-size: 17px;
    line-height: 17px;
    color: red;

    /* Better Font Rendering =========== */
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  &.icon-chat:before {
    content: "\\e904";
  }

  &.icon-check:before {
    content: "\\e905";
  }
`;