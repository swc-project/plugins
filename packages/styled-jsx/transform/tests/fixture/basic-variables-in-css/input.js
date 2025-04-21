export default function Component() {
  const width = 100;
  const height = 200;
  const color = '#FF0000';

  return (
    <div>
      <h1>Basic Variables Test</h1>
      <style jsx>{`
        .component {
          width: ${width}px;
          height: ${height}px;
          color: ${color};
        }
      `}</style>
    </div>
  );
}