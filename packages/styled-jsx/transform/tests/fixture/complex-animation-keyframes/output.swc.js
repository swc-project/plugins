import _JSXStyle from "styled-jsx/style";
export default function Component() {
    const middleOpacity = 0.5;
    const rotation = 180;
    const duration = 1000;
    const easing = 'ease-in-out';
    const delay = 200;
    return <div className={_JSXStyle.dynamic([
        [
            "e4a36e7183ef0610",
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
            "e4a36e7183ef0610",
            [
                middleOpacity,
                rotation,
                duration,
                easing,
                delay
            ]
        ]
    ]) + " " + "animated"}>Animated Element</div>
      <_JSXStyle id={"e4a36e7183ef0610"} dynamic={[
        middleOpacity,
        rotation,
        duration,
        easing,
        delay
    ]}>{`@-webkit-keyframes customAnimation{0%{opacity:0;-webkit-transform:scale(0);transform:scale(0)}50%{opacity:${middleOpacity};-webkit-transform:rotate(${rotation}deg);transform:rotate(${rotation}deg)}to{opacity:1;-webkit-transform:scale(1);transform:scale(1)}}@-moz-keyframes customAnimation{0%{opacity:0;-moz-transform:scale(0);transform:scale(0)}50%{opacity:${middleOpacity};-moz-transform:rotate(${rotation}deg);transform:rotate(${rotation}deg)}to{opacity:1;-moz-transform:scale(1);transform:scale(1)}}@-o-keyframes customAnimation{0%{opacity:0;-o-transform:scale(0);transform:scale(0)}50%{opacity:${middleOpacity};-o-transform:rotate(${rotation}deg);transform:rotate(${rotation}deg)}to{opacity:1;-o-transform:scale(1);transform:scale(1)}}@keyframes customAnimation{0%{opacity:0;-webkit-transform:scale(0);-moz-transform:scale(0);-o-transform:scale(0);transform:scale(0)}50%{opacity:${middleOpacity};-webkit-transform:rotate(${rotation}deg);-moz-transform:rotate(${rotation}deg);-o-transform:rotate(${rotation}deg);transform:rotate(${rotation}deg)}to{opacity:1;-webkit-transform:scale(1);-moz-transform:scale(1);-o-transform:scale(1);transform:scale(1)}}.wrapper.__jsx-style-dynamic-selector{}.wrapper.__jsx-style-dynamic-selector .animated.__jsx-style-dynamic-selector{-webkit-animation:customAnimation ${duration}ms ${easing} forwards;-moz-animation:customAnimation ${duration}ms ${easing} forwards;-o-animation:customAnimation ${duration}ms ${easing} forwards;animation:customAnimation ${duration}ms ${easing} forwards;-webkit-animation-delay:${delay}ms;-moz-animation-delay:${delay}ms;-o-animation-delay:${delay}ms;animation-delay:${delay}ms}`}</_JSXStyle>
    </div>;
}
