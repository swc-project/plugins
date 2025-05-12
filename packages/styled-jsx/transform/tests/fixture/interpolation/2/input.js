export default function Home() {
  const breakpoint = "500px";
  return (
    <div>
      <div className="container">
        container (should be blue)
        <div className="responsive">
          responsive (purple on mobile, orange on desktop)
        </div>
      </div>

      <style jsx>{`
          .container {
            color: blue;
            padding: 3rem;
  
            @media (max-width: ${breakpoint}) {
              .responsive {
                color: purple;
              }
            }
          }
        `}</style>
    </div>
  );
}