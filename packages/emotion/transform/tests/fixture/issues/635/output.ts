import { jsx as _jsx } from "react/jsx-runtime";
import { css } from "@emotion/react";
import { forwardRef } from "react";
const styles = {
    row: (theme)=>/*#__PURE__*/ css({
            display: "grid",
            gap: theme.spacing.sm
        }, "label:row", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaW5wdXQudHMiLCJzb3VyY2VzIjpbImlucHV0LnRzIl0sInNvdXJjZXNDb250ZW50IjpbImltcG9ydCB7IGNzcyB9IGZyb20gXCJAZW1vdGlvbi9yZWFjdFwiO1xuaW1wb3J0IHsgZm9yd2FyZFJlZiB9IGZyb20gXCJyZWFjdFwiO1xuXG5jb25zdCBzdHlsZXMgPSB7XG4gIHJvdzogKHRoZW1lKSA9PiBjc3MoeyBkaXNwbGF5OiBcImdyaWRcIiwgZ2FwOiB0aGVtZS5zcGFjaW5nLnNtIH0pLFxufTtcblxuZXhwb3J0IGNvbnN0IFJvd1dpdGhTcHJlYWQgPSBmb3J3YXJkUmVmKChwcm9wcywgcmVmKSA9PiAoXG4gIDxkaXYgcmVmPXtyZWZ9IGNzcz17W3N0eWxlcy5yb3csIHt9XX0gey4uLnByb3BzfSAvPlxuKSk7XG5cbmV4cG9ydCBmdW5jdGlvbiBSb3dXaXRob3V0U3ByZWFkKCkge1xuICByZXR1cm4gPGRpdiBjc3M9e1tzdHlsZXMucm93LCB7fV19IC8+O1xufVxuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUlrQiJ9 */")
};
export const RowWithSpread = forwardRef((props, ref)=>/*#__PURE__*/ _jsx("div", {
        ref: ref,
        css: [
            styles.row,
            {}
        ],
        ...props
    }));
export function RowWithoutSpread() {
    return /*#__PURE__*/ _jsx("div", {
        css: [
            styles.row,
            {}
        ]
    });
}
