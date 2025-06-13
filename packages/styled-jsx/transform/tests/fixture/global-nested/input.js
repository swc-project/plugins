const Test = () => (
    <div>
        <span>test</span>
        <style jsx>{`
        .p1 {
          background: purple !important;

          :global(.c1),
          .c2 {
            background: orange !important;
          }
        }

      `}</style>
    </div>
);
