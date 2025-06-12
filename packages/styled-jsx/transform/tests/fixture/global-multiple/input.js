const Test = () => (
  <div>
    <span>test</span>
    <style jsx>{`
        :global(.c1, .c2) {
          background: orange !important;
        }
      `}</style>
  </div>
);
