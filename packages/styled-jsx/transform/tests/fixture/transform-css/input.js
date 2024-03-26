export default () => (
  <div>
    <p>test</p>
    <style jsx>{`
      html {
        background-image: linear-gradient(
            0deg,
            rgba(255, 255, 255, 0.8),
            rgba(255, 255, 255, 0.8)
          ),
          url(/static/background.svg);
      }

      :global(p) {
        color: #001;
      }

      :global(p) {
        color: #002;
      }

      :global(p),
      a {
        color: #003;
      }

      :global(.foo + a) {
        color: #004;
      }

      :global(body) {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica,
          Arial, sans-serif;
      }

      p {
        color: #005;
      }

      p {
        color: #006;
      }

      * {
        color: #007;
      }

      [href="woot"] {
        color: #008;
      }

      p a span {
        color: #009;
      }

      p :global(span) {
        background: #00a;
      }

      p a[title="'w ' '  t'"] {
        margin: auto;
      }

      p :global(span:not(.test)) {
        color: #00b;
      }

      p,
      h1 {
        color: #00c;
        animation: hahaha 3s ease forwards infinite;
        animation-name: hahaha;
        animation-delay: 100ms;
      }

      p {
        animation:
          hahaha 1s,
          hehehe 2s;
      }

      p:hover {
        color: #00d;
      }

      p::before {
        color: #00e;
      }

      :hover {
        color: #00f;
      }

      ::before {
        color: #010;
      }

      :hover p {
        color: #011;
      }

      p + a {
        color: #012;
      }

      p ~ a {
        color: #013;
      }

      p > a {
        color: #014;
      }

      @keyframes hahaha {
        from {
          top: 0;
        }
        to {
          top: 100;
        }
      }

      @keyframes hehehe {
        from {
          left: 0;
        }
        to {
          left: 100;
        }
      }

      @media (min-width: 500px) {
        .test {
          color: #015;
        }
      }

      .test {
        /* test, test */
        display: block;
        /*
      
        test
        */
      }

      .inline-flex {
        display: inline-flex;
      }

      .flex {
        display: flex;
      }

      .test {
        box-shadow:
          0 0 10px black,
          inset 0 0 5px black;
      }

      .test[title=","] {
        display: inline-block;
      }

      .test.is-status .test {
        color: #016;
      }

      .a-selector:hover,
      .a-selector:focus {
        outline: none;
      }

      @media (min-width: 1px) and (max-width: 768px) {
        [class*="grid__col--"] {
          margin-top: 12px;
          margin-bottom: 12px;
        }
      }

      @media (max-width: 64em) {
        .test {
          margin-bottom: 1em;
        }
        @supports (-moz-appearance: none) and (display: contents) {
          .test {
            margin-bottom: 2rem;
          }
        }
      }
    `}</style>
  </div>
);
