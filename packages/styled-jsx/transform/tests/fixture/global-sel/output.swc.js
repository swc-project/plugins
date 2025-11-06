import _JSXStyle from "styled-jsx/style";
const Test = ()=><>
    <div className={"jsx-60bae1dc4409a62b" + " " + "container"}>
      <div className={"jsx-60bae1dc4409a62b" + " " + "p1"}>
        .p1 -This is another parent.
        <div className={"jsx-60bae1dc4409a62b" + " " + "c1"}>
          .c1 - This should be orange bg.
          <div className={"jsx-60bae1dc4409a62b" + " " + "c2"}>.c2 - This should be orange bg as well.</div>
        </div>
      </div>

      <hr className={"jsx-60bae1dc4409a62b" + " " + "my-4"}/>

      <h3 className={"jsx-60bae1dc4409a62b" + " " + "mb-2"}>Compiled CSS</h3>
      {compiledCss ? <pre className={"jsx-60bae1dc4409a62b"}>{formatCss(compiledCss)}</pre> : <p className={"jsx-60bae1dc4409a62b"}>Loading CSS...</p>}
    </div>

    <_JSXStyle id={"60bae1dc4409a62b"}>{"#css-test-page-id.jsx-60bae1dc4409a62b{color:red}.p1.jsx-60bae1dc4409a62b{background:purple!important}.c1,.c2.jsx-60bae1dc4409a62b{background:orange!important}.container.jsx-60bae1dc4409a62b{max-width:1000px;margin:0 auto;padding:1rem}"}</_JSXStyle>
  </>;
