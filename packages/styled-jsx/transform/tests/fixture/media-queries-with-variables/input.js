export default function Component() {
  const ResponsiveBreakpoint = {
    mobile: '768px',
    tablet: '1024px',
    desktop: '1440px'
  };
  const breakpoint = 'mobile';
  const mobileWidth = 320;

  return (
    <div className="component">
      <div className="active">Responsive Element</div>
      <style jsx>{`
        .component {
          width: 100%;

          @media (max-width: ${ResponsiveBreakpoint[breakpoint]}) {
            width: ${mobileWidth}px;

            &.active {
              color: blue;
            }
            
            div {
              display: block;
            }
          }
        }
      `}</style>
    </div>
  );
}