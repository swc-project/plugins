const Test = () => (
  <div>
    <span>test</span>
    <style jsx>{`
        .parent {
          :global(div) {
            :global(&.child, &.child2) {
              background: orange;
            }
          }
        }
      `}</style>
  </div>
);
