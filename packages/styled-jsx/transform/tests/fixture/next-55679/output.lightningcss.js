import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>

      <_JSXStyle id={"f559f0d916e6258b"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>

      <Component {...pageProps} className={"jsx-f559f0d916e6258b" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>

    </>;
}
