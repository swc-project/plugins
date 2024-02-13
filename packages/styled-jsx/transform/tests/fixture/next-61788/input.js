const MOBILE_MAX = 767;

export default function Home() {
  return (
    <div>
      <h1 className="header">Hello</h1>
      <style jsx>{`
        .header {
          font-size: 48px;
        }

        @media screen and (max-width: ${MOBILE_MAX}px) {
          .header {
            font-size: 12px;
          }
        }
      `}</style>
    </div>
  );
}
