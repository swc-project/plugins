import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>
      <_JSXStyle id={"a787ac3dd938bf9f"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>
      <Component {...pageProps} className={"jsx-a787ac3dd938bf9f" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>
    </>;
}
