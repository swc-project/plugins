import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>
      <_JSXStyle id={"6ac6a995dbb24dba"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>
      <Component {...pageProps} className={"jsx-6ac6a995dbb24dba" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>
    </>;
}
