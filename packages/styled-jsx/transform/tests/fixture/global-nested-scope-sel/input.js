const Test = () => (
  <div>
    <span>test</span>
    <style jsx>{`
        .parent {
          :global(div) {
            :global(&.child) {
              background: orange;
            }
          }
        }
      `}</style>
  </div>
);
