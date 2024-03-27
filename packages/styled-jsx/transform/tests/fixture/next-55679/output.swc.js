import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>

      <_JSXStyle id={"caeea3e7c6bbdab1"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>

      <Component {...pageProps} className={"jsx-caeea3e7c6bbdab1" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>

    </>;
}
