import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>
      <_JSXStyle id={"3325ddb654ec5bd1"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>
      <Component {...pageProps} className={"jsx-3325ddb654ec5bd1" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>
    </>;
}
