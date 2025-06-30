const Test = () => (
  <>
    <div className="container">
      <div className="p1">
        .p1 -This is another parent.
        <div className="c1">
          .c1 - This should be orange bg.
          <div className="c2">.c2 - This should be orange bg as well.</div>
        </div>
      </div>

      <hr className="my-4" />

      <h3 className="mb-2">Compiled CSS</h3>
      {compiledCss ? (
        <pre>{formatCss(compiledCss)}</pre>
      ) : (
        <p>Loading CSS...</p>
      )}
    </div>

    <style jsx>{`
        // This does nothing except helping the JS find the StyledJSX element on the page.
        #css-test-page-id {
          color: red;
        }

        .p1 {
          background: purple !important;
        }

        :global(.c1),
        .c2 {
          background: orange !important;
        }

        .container {
          max-width: 1000px;
          margin: 0 auto;
          padding: 1rem;
        }
      `}</style>
  </>
);
