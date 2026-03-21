import { jsx as _jsx } from "react/jsx-runtime";
import { css } from "@emotion/react";
import { PureComponent } from "react";
export class SimpleComponent extends PureComponent {
    render() {
        return /*#__PURE__*/ _jsx("div", {
            css: css({
                color: "red"
            }, "label:SimpleComponent", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaW5wdXQudHMiLCJzb3VyY2VzIjpbImlucHV0LnRzIl0sInNvdXJjZXNDb250ZW50IjpbImltcG9ydCB7IGNzcyB9IGZyb20gXCJAZW1vdGlvbi9yZWFjdFwiO1xuaW1wb3J0IHsgUHVyZUNvbXBvbmVudCB9IGZyb20gXCJyZWFjdFwiO1xuXG5leHBvcnQgY2xhc3MgU2ltcGxlQ29tcG9uZW50IGV4dGVuZHMgUHVyZUNvbXBvbmVudCB7XG4gIHJlbmRlcigpIHtcbiAgICByZXR1cm4gKFxuICAgICAgPGRpdiBjc3M9e3sgY29sb3I6IFwicmVkXCIgfX0+SGVsbG88L2Rpdj5cbiAgICApO1xuICB9XG59XG5cbi8vIEFsc28gdGVzdCB3aXRoIGFycmF5IG9mIG9iamVjdHNcbmV4cG9ydCBmdW5jdGlvbiBBcnJheUNzc1Byb3AoKSB7XG4gIHJldHVybiA8ZGl2IGNzcz17W3sgY29sb3I6IFwiYmx1ZVwiIH0sIHsgYmFja2dyb3VuZDogXCJ3aGl0ZVwiIH1dfT5Xb3JsZDwvZGl2Pjtcbn1cbiJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFNZSJ9 */"),
            children: "Hello"
        });
    }
}
// Also test with array of objects
export function ArrayCssProp() {
    return /*#__PURE__*/ _jsx("div", {
        css: css([
            {
                color: "blue"
            },
            {
                background: "white"
            }
        ], "label:ArrayCssProp", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaW5wdXQudHMiLCJzb3VyY2VzIjpbImlucHV0LnRzIl0sInNvdXJjZXNDb250ZW50IjpbImltcG9ydCB7IGNzcyB9IGZyb20gXCJAZW1vdGlvbi9yZWFjdFwiO1xuaW1wb3J0IHsgUHVyZUNvbXBvbmVudCB9IGZyb20gXCJyZWFjdFwiO1xuXG5leHBvcnQgY2xhc3MgU2ltcGxlQ29tcG9uZW50IGV4dGVuZHMgUHVyZUNvbXBvbmVudCB7XG4gIHJlbmRlcigpIHtcbiAgICByZXR1cm4gKFxuICAgICAgPGRpdiBjc3M9e3sgY29sb3I6IFwicmVkXCIgfX0+SGVsbG88L2Rpdj5cbiAgICApO1xuICB9XG59XG5cbi8vIEFsc28gdGVzdCB3aXRoIGFycmF5IG9mIG9iamVjdHNcbmV4cG9ydCBmdW5jdGlvbiBBcnJheUNzc1Byb3AoKSB7XG4gIHJldHVybiA8ZGl2IGNzcz17W3sgY29sb3I6IFwiYmx1ZVwiIH0sIHsgYmFja2dyb3VuZDogXCJ3aGl0ZVwiIH1dfT5Xb3JsZDwvZGl2Pjtcbn1cbiJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFha0IifQ== */"),
        children: "World"
    });
}
