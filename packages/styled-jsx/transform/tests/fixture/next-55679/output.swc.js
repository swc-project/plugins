import _JSXStyle from "styled-jsx/style";
import { AppProps } from "next/app";
const someVar = "--var-1";
export default function App({ Component, pageProps }) {
    return <>
      <_JSXStyle id={"ec57ae9caf990f49"}>{`:root{${someVar}:red;background-color:var(${someVar})}`}</_JSXStyle>
      <Component {...pageProps} className={"jsx-ec57ae9caf990f49" + " " + (pageProps && pageProps.className != null && pageProps.className || "")}/>
    </>;
}
