import _JSXStyle from "swc-magic/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>

            <_JSXStyle id={"e26e4619775cb212"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>

            <Component {...pageProps} className={"jsx-e26e4619775cb212" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>

        </>;
}
