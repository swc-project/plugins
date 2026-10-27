import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>
      <_JSXStyle id={"91a83de9e4159ddc"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>
      <Component {...pageProps} className={"jsx-91a83de9e4159ddc" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>
    </>;
}
