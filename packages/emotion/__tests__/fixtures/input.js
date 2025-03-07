const myStyles = css`
  font-size: 20px;
  @media (min-width: 420px) {
    color: blue;
    ${css`
      width: 96px;
      height: 96px;
    `};
    line-height: 26px;
  }
  background: green;
  ${{ backgroundColor: "hotpink" }};
`;
