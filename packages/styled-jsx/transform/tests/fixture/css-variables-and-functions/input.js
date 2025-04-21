export default function Component() {
  const dynamicValue = '"dynamic content"';
  const color1 = '#FF0000';
  const color2 = '#0000FF';
  const offset = 5;
  const spacing = 10;

  return (
    <div className="container">
      <div className="item">CSS Variables and Functions</div>
      <style jsx>{`
        .container {
          --local-var: ${dynamicValue};
          color: var(--text-color);
          background: linear-gradient(to right, ${color1}, ${color2});

          .item {
            transform: translate(
              calc(var(--x) + ${offset}px),
              calc(var(--y) + ${offset}px)
            );
          }
          
          div {
            margin: calc(10px + ${spacing}px);
          }
        }
      `}</style>
    </div>
  );
}