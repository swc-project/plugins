import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>
      <_JSXStyle id={"78a0e23939332fdf"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>
      <Component {...pageProps} className={"jsx-78a0e23939332fdf" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>
    </>;
}
