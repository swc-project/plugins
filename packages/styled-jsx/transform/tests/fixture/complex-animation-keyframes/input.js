export default function Component() {
  const middleOpacity = 0.5;
  const rotation = 180;
  const duration = 1000;
  const easing = 'ease-in-out';
  const delay = 200;

  return (
    <div className="wrapper">
      <div className="animated">Animated Element</div>
      <style jsx>{`
        @keyframes customAnimation {
          0% {
            opacity: 0;
            transform: scale(0);
          }
          50% {
            opacity: ${middleOpacity};
            transform: rotate(${rotation}deg);
          }
          100% {
            opacity: 1;
            transform: scale(1);
          }
        }
        .wrapper {
          .animated {
            animation: customAnimation ${duration}ms ${easing} forwards;
            animation-delay: ${delay}ms;
          }
        }
      `}</style>
    </div>
  );
}