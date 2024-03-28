// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/inside-class.js
import { jsx as _jsx } from "react/jsx-runtime";
import { Component } from "react";
import { jsx, css } from "@emotion/react";
class SomeComponent extends Component {
    member = /*#__PURE__*/ css("color:hotpink;", "member", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9pbnNpZGUtY2xhc3MudHN4Iiwic291cmNlcyI6WyJlbW90aW9uLWpzL2luc2lkZS1jbGFzcy50c3giXSwic291cmNlc0NvbnRlbnQiOlsiLy8gaHR0cHM6Ly9naXRodWIuY29tL2Vtb3Rpb24tanMvZW1vdGlvbi9ibG9iL21haW4vcGFja2FnZXMvYmFiZWwtcGx1Z2luL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL2luc2lkZS1jbGFzcy5qc1xuXG5pbXBvcnQgeyBDb21wb25lbnQgfSBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCB7IGpzeCwgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmNsYXNzIFNvbWVDb21wb25lbnQgZXh0ZW5kcyBDb21wb25lbnQge1xuICBtZW1iZXIgPSBjc3NgXG4gICAgY29sb3I6IGhvdHBpbms7XG4gIGA7XG5cbiAgcmVuZGVyKCkge1xuICAgIHJldHVybiAoXG4gICAgICA8ZGl2XG4gICAgICAgIGNzcz17Y3NzYFxuICAgICAgICAgIGNvbG9yOiBob3RwaW5rO1xuICAgICAgICBgfVxuICAgICAgLz5cbiAgICApO1xuICB9XG59XG5cbmNsYXNzIFNvbWVPdGhlckNvbXBvbmVudCBleHRlbmRzIENvbXBvbmVudCB7XG4gIG1lbWJlciA9IGNzc2BcbiAgICBjb2xvcjogaG90cGluaztcbiAgYDtcblxuICByZW5kZXIoKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDxkaXZcbiAgICAgICAgY3NzPXtjc3NgXG4gICAgICAgICAgY29sb3I6IGhvdHBpbms7XG5cbiAgICAgICAgICAuZm9vIHtcbiAgICAgICAgICAgICR7dGhpcy5tZW1iZXJ9XG4gICAgICAgICAgfVxuICAgICAgICBgfVxuICAgICAgLz5cbiAgICApO1xuICB9XG59XG4iXSwibmFtZXMiOltdLCJyYW5nZU1hcHBpbmdzIjoiIiwibWFwcGluZ3MiOiJBQU1XIn0= */");
    render() {
        return /*#__PURE__*/ _jsx("div", {
            css: /*#__PURE__*/ css("color:hotpink;", "SomeComponent", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9pbnNpZGUtY2xhc3MudHN4Iiwic291cmNlcyI6WyJlbW90aW9uLWpzL2luc2lkZS1jbGFzcy50c3giXSwic291cmNlc0NvbnRlbnQiOlsiLy8gaHR0cHM6Ly9naXRodWIuY29tL2Vtb3Rpb24tanMvZW1vdGlvbi9ibG9iL21haW4vcGFja2FnZXMvYmFiZWwtcGx1Z2luL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL2luc2lkZS1jbGFzcy5qc1xuXG5pbXBvcnQgeyBDb21wb25lbnQgfSBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCB7IGpzeCwgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmNsYXNzIFNvbWVDb21wb25lbnQgZXh0ZW5kcyBDb21wb25lbnQge1xuICBtZW1iZXIgPSBjc3NgXG4gICAgY29sb3I6IGhvdHBpbms7XG4gIGA7XG5cbiAgcmVuZGVyKCkge1xuICAgIHJldHVybiAoXG4gICAgICA8ZGl2XG4gICAgICAgIGNzcz17Y3NzYFxuICAgICAgICAgIGNvbG9yOiBob3RwaW5rO1xuICAgICAgICBgfVxuICAgICAgLz5cbiAgICApO1xuICB9XG59XG5cbmNsYXNzIFNvbWVPdGhlckNvbXBvbmVudCBleHRlbmRzIENvbXBvbmVudCB7XG4gIG1lbWJlciA9IGNzc2BcbiAgICBjb2xvcjogaG90cGluaztcbiAgYDtcblxuICByZW5kZXIoKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDxkaXZcbiAgICAgICAgY3NzPXtjc3NgXG4gICAgICAgICAgY29sb3I6IGhvdHBpbms7XG5cbiAgICAgICAgICAuZm9vIHtcbiAgICAgICAgICAgICR7dGhpcy5tZW1iZXJ9XG4gICAgICAgICAgfVxuICAgICAgICBgfVxuICAgICAgLz5cbiAgICApO1xuICB9XG59XG4iXSwibmFtZXMiOltdLCJyYW5nZU1hcHBpbmdzIjoiIiwibWFwcGluZ3MiOiJBQWFhIn0= */")
        });
    }
}
class SomeOtherComponent extends Component {
    member = /*#__PURE__*/ css("color:hotpink;", "member", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9pbnNpZGUtY2xhc3MudHN4Iiwic291cmNlcyI6WyJlbW90aW9uLWpzL2luc2lkZS1jbGFzcy50c3giXSwic291cmNlc0NvbnRlbnQiOlsiLy8gaHR0cHM6Ly9naXRodWIuY29tL2Vtb3Rpb24tanMvZW1vdGlvbi9ibG9iL21haW4vcGFja2FnZXMvYmFiZWwtcGx1Z2luL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL2luc2lkZS1jbGFzcy5qc1xuXG5pbXBvcnQgeyBDb21wb25lbnQgfSBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCB7IGpzeCwgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmNsYXNzIFNvbWVDb21wb25lbnQgZXh0ZW5kcyBDb21wb25lbnQge1xuICBtZW1iZXIgPSBjc3NgXG4gICAgY29sb3I6IGhvdHBpbms7XG4gIGA7XG5cbiAgcmVuZGVyKCkge1xuICAgIHJldHVybiAoXG4gICAgICA8ZGl2XG4gICAgICAgIGNzcz17Y3NzYFxuICAgICAgICAgIGNvbG9yOiBob3RwaW5rO1xuICAgICAgICBgfVxuICAgICAgLz5cbiAgICApO1xuICB9XG59XG5cbmNsYXNzIFNvbWVPdGhlckNvbXBvbmVudCBleHRlbmRzIENvbXBvbmVudCB7XG4gIG1lbWJlciA9IGNzc2BcbiAgICBjb2xvcjogaG90cGluaztcbiAgYDtcblxuICByZW5kZXIoKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDxkaXZcbiAgICAgICAgY3NzPXtjc3NgXG4gICAgICAgICAgY29sb3I6IGhvdHBpbms7XG5cbiAgICAgICAgICAuZm9vIHtcbiAgICAgICAgICAgICR7dGhpcy5tZW1iZXJ9XG4gICAgICAgICAgfVxuICAgICAgICBgfVxuICAgICAgLz5cbiAgICApO1xuICB9XG59XG4iXSwibmFtZXMiOltdLCJyYW5nZU1hcHBpbmdzIjoiIiwibWFwcGluZ3MiOiJBQXNCVyJ9 */");
    render() {
        return /*#__PURE__*/ _jsx("div", {
            css: /*#__PURE__*/ css("color:hotpink;.foo{", this.member, "}", "SomeOtherComponent", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9pbnNpZGUtY2xhc3MudHN4Iiwic291cmNlcyI6WyJlbW90aW9uLWpzL2luc2lkZS1jbGFzcy50c3giXSwic291cmNlc0NvbnRlbnQiOlsiLy8gaHR0cHM6Ly9naXRodWIuY29tL2Vtb3Rpb24tanMvZW1vdGlvbi9ibG9iL21haW4vcGFja2FnZXMvYmFiZWwtcGx1Z2luL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL2luc2lkZS1jbGFzcy5qc1xuXG5pbXBvcnQgeyBDb21wb25lbnQgfSBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCB7IGpzeCwgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmNsYXNzIFNvbWVDb21wb25lbnQgZXh0ZW5kcyBDb21wb25lbnQge1xuICBtZW1iZXIgPSBjc3NgXG4gICAgY29sb3I6IGhvdHBpbms7XG4gIGA7XG5cbiAgcmVuZGVyKCkge1xuICAgIHJldHVybiAoXG4gICAgICA8ZGl2XG4gICAgICAgIGNzcz17Y3NzYFxuICAgICAgICAgIGNvbG9yOiBob3RwaW5rO1xuICAgICAgICBgfVxuICAgICAgLz5cbiAgICApO1xuICB9XG59XG5cbmNsYXNzIFNvbWVPdGhlckNvbXBvbmVudCBleHRlbmRzIENvbXBvbmVudCB7XG4gIG1lbWJlciA9IGNzc2BcbiAgICBjb2xvcjogaG90cGluaztcbiAgYDtcblxuICByZW5kZXIoKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDxkaXZcbiAgICAgICAgY3NzPXtjc3NgXG4gICAgICAgICAgY29sb3I6IGhvdHBpbms7XG5cbiAgICAgICAgICAuZm9vIHtcbiAgICAgICAgICAgICR7dGhpcy5tZW1iZXJ9XG4gICAgICAgICAgfVxuICAgICAgICBgfVxuICAgICAgLz5cbiAgICApO1xuICB9XG59XG4iXSwibmFtZXMiOltdLCJyYW5nZU1hcHBpbmdzIjoiIiwibWFwcGluZ3MiOiJBQTZCYSJ9 */")
        });
    }
}