export default function Component() {
  return (
    <div className="parent">
      <div className="child">Nested Selectors Test</div>
      <h1>Heading</h1>
      <style jsx>{`
        .parent {
          position: relative;

          &:hover {
            background-color: red;
          }

          .child {
            margin-top: 10px;
          }
          
          div {
            padding: 15px;
          }
          
          h1 {
            font-size: 24px;
          }
        }
      `}</style>
    </div>
  );
}