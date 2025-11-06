import _JSXStyle from "styled-jsx/style";
export default function Component() {
    const middleOpacity = 0.5;
    const rotation = 180;
    const duration = 1000;
    const easing = 'ease-in-out';
    const delay = 200;
    return <div className={_JSXStyle.dynamic([
        [
            "30b0abc2151c644d",
            [
                middleOpacity,
                rotation,
                duration,
                easing,
                delay
            ]
        ]
    ]) + " " + "wrapper"}>
      <div className={_JSXStyle.dynamic([
        [
            "30b0abc2151c644d",
            [
                middleOpacity,
                rotation,
                duration,
                easing,
                delay
            ]
        ]
    ]) + " " + "animated"}>Animated Element</div>
      <_JSXStyle id={"30b0abc2151c644d"} dynamic={[
        middleOpacity,
        rotation,
        duration,
        easing,
        delay
    ]}>{`@keyframes customAnimation{0%{opacity:0;transform:scale(0)}50%{opacity:${middleOpacity};transform:rotate(${rotation}deg)}to{opacity:1;transform:scale(1)}}.wrapper.__jsx-style-dynamic-selector .animated.__jsx-style-dynamic-selector{animation:customAnimation ${duration}ms ${easing} forwards;animation-delay:${delay}ms}`}</_JSXStyle>
    </div>;
}
