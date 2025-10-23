import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>
      <_JSXStyle id={"38fdbc96e669f61f"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>
      <Component {...pageProps} className={"jsx-38fdbc96e669f61f" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>
    </>;
}
