import { AppProps } from "next/app";

const someVar = "--var-1";

export default function App({ Component, pageProps }) {
    return (
        <>
            <style jsx global>{`
        :root {
          ${someVar}: red;
          background-color: var(${someVar});
        }
      `}</style>
            <Component {...pageProps} />
        </>
    );
}