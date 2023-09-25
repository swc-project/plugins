import { AppProps } from "next/app";

const someVar = "--var-1";

export default function App({ Component, pageProps }: AppProps) {
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