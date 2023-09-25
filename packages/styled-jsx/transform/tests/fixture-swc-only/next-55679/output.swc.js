import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>

            <_JSXStyle id={"17fa9359df3478d8"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>

            <Component {...pageProps} className={"jsx-17fa9359df3478d8" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>

        </>;
}
