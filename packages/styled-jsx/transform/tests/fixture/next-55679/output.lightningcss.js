import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>
      <_JSXStyle id={"375c0739fd480132"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>
      <Component {...pageProps} className={"jsx-375c0739fd480132" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>
    </>;
}
