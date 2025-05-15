const Test = () => (
  <div>
    <div className="child">Child div</div>
    <style jsx>{`
      .container {
        color: blue;
        padding: 3rem;

        :global(div.child) {
          color: green;
        }
      }
    `}</style>
  </div>
);