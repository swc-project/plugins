import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>
      <_JSXStyle id={"decfccc0ebccf98f"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>
      <Component {...pageProps} className={"jsx-decfccc0ebccf98f" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>
    </>;
}
