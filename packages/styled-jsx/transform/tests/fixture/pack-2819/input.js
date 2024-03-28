export default () => (
  <div>
    <p>test</p>
    <p>woot</p>
    <p>woot</p>
    <style jsx>{`
      .container {
        color: blue;
        padding: 3rem;

        &.inner {
          color: yellow;
        }
      }
    `}</style>
  </div>
);
