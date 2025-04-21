import _JSXStyle from "styled-jsx/style";
export default function Component() {
    const middleOpacity = 0.5;
    const rotation = 180;
    const duration = 1000;
    const easing = 'ease-in-out';
    const delay = 200;
    return <div className={_JSXStyle.dynamic([
        [
            "18cb1cdcd10135a3",
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
            "18cb1cdcd10135a3",
            [
                middleOpacity,
                rotation,
                duration,
                easing,
                delay
            ]
        ]
    ]) + " " + "animated"}>Animated Element</div>
      <_JSXStyle id={"18cb1cdcd10135a3"} dynamic={[
        middleOpacity,
        rotation,
        duration,
        easing,
        delay
    ]}>{`.wrapper.__jsx-style-dynamic-selector{}.wrapper.__jsx-style-dynamic-selector .animated.__jsx-style-dynamic-selector{-webkit-animation:customAnimation ${duration}ms ${easing} forwards;-moz-animation:customAnimation ${duration}ms ${easing} forwards;-o-animation:customAnimation ${duration}ms ${easing} forwards;animation:customAnimation ${duration}ms ${easing} forwards;-webkit-animation-delay:${delay}ms;-moz-animation-delay:${delay}ms;-o-animation-delay:${delay}ms;animation-delay:${delay}ms}`}</_JSXStyle>
    </div>;
}
