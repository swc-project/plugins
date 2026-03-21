import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime";
import * as emotionReact from "@emotion/react";
import { PureComponent } from "react";
import ReactDOM from "react-dom";
const stylesInCallback = (props: any)=>/*#__PURE__*/ emotionReact.css({
        color: "red",
        background: "yellow",
        width: `${props.scale * 100}px`
    }, "label:stylesInCallback", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaW5wdXQudHMiLCJzb3VyY2VzIjpbImlucHV0LnRzIl0sInNvdXJjZXNDb250ZW50IjpbImltcG9ydCAqIGFzIGVtb3Rpb25SZWFjdCBmcm9tIFwiQGVtb3Rpb24vcmVhY3RcIjtcbmltcG9ydCB7IFB1cmVDb21wb25lbnQgfSBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCBSZWFjdERPTSBmcm9tIFwicmVhY3QtZG9tXCI7XG5cbmNvbnN0IHN0eWxlc0luQ2FsbGJhY2sgPSAocHJvcHM6IGFueSkgPT5cbiAgZW1vdGlvblJlYWN0LmNzcyh7XG4gICAgY29sb3I6IFwicmVkXCIsXG4gICAgYmFja2dyb3VuZDogXCJ5ZWxsb3dcIixcbiAgICB3aWR0aDogYCR7cHJvcHMuc2NhbGUgKiAxMDB9cHhgLFxuICB9KTtcblxuY29uc3Qgc3R5bGVzID0gZW1vdGlvblJlYWN0LmNzcyh7XG4gIGNvbG9yOiBcInJlZFwiLFxuICB3aWR0aDogXCIyMHB4XCIsXG59KTtcblxuY29uc3Qgc3R5bGVzMiA9IGVtb3Rpb25SZWFjdC5jc3NgXG4gIGNvbG9yOiByZWQ7XG4gIHdpZHRoOiAyMHB4O1xuYDtcblxuZXhwb3J0IGNsYXNzIFNpbXBsZUNvbXBvbmVudCBleHRlbmRzIFB1cmVDb21wb25lbnQge1xuICByZW5kZXIoKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDw+XG4gICAgICAgIDxkaXYgY2xhc3NOYW1lPXtzdHlsZXN9PlxuICAgICAgICAgIDxzcGFuPmhlbGxvPC9zcGFuPlxuICAgICAgICA8L2Rpdj5cbiAgICAgICAgPGRpdiBjc3M9e3sgY29sb3I6IFwiYmx1ZVwiIH19PmNzcyBwcm9wPC9kaXY+XG4gICAgICA8Lz5cbiAgICApO1xuICB9XG59XG5cblJlYWN0RE9NLnJlbmRlcig8U2ltcGxlQ29tcG9uZW50IC8+LCBkb2N1bWVudC5xdWVyeVNlbGVjdG9yKFwiI2FwcFwiKSk7XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBS0UifQ== */");
const styles = /*#__PURE__*/ emotionReact.css({
    color: "red",
    width: "20px"
}, "label:styles", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaW5wdXQudHMiLCJzb3VyY2VzIjpbImlucHV0LnRzIl0sInNvdXJjZXNDb250ZW50IjpbImltcG9ydCAqIGFzIGVtb3Rpb25SZWFjdCBmcm9tIFwiQGVtb3Rpb24vcmVhY3RcIjtcbmltcG9ydCB7IFB1cmVDb21wb25lbnQgfSBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCBSZWFjdERPTSBmcm9tIFwicmVhY3QtZG9tXCI7XG5cbmNvbnN0IHN0eWxlc0luQ2FsbGJhY2sgPSAocHJvcHM6IGFueSkgPT5cbiAgZW1vdGlvblJlYWN0LmNzcyh7XG4gICAgY29sb3I6IFwicmVkXCIsXG4gICAgYmFja2dyb3VuZDogXCJ5ZWxsb3dcIixcbiAgICB3aWR0aDogYCR7cHJvcHMuc2NhbGUgKiAxMDB9cHhgLFxuICB9KTtcblxuY29uc3Qgc3R5bGVzID0gZW1vdGlvblJlYWN0LmNzcyh7XG4gIGNvbG9yOiBcInJlZFwiLFxuICB3aWR0aDogXCIyMHB4XCIsXG59KTtcblxuY29uc3Qgc3R5bGVzMiA9IGVtb3Rpb25SZWFjdC5jc3NgXG4gIGNvbG9yOiByZWQ7XG4gIHdpZHRoOiAyMHB4O1xuYDtcblxuZXhwb3J0IGNsYXNzIFNpbXBsZUNvbXBvbmVudCBleHRlbmRzIFB1cmVDb21wb25lbnQge1xuICByZW5kZXIoKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDw+XG4gICAgICAgIDxkaXYgY2xhc3NOYW1lPXtzdHlsZXN9PlxuICAgICAgICAgIDxzcGFuPmhlbGxvPC9zcGFuPlxuICAgICAgICA8L2Rpdj5cbiAgICAgICAgPGRpdiBjc3M9e3sgY29sb3I6IFwiYmx1ZVwiIH19PmNzcyBwcm9wPC9kaXY+XG4gICAgICA8Lz5cbiAgICApO1xuICB9XG59XG5cblJlYWN0RE9NLnJlbmRlcig8U2ltcGxlQ29tcG9uZW50IC8+LCBkb2N1bWVudC5xdWVyeVNlbGVjdG9yKFwiI2FwcFwiKSk7XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBV2UifQ== */");
const styles2 = /*#__PURE__*/ emotionReact.css("color:red;width:20px;", "label:styles2;", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaW5wdXQudHMiLCJzb3VyY2VzIjpbImlucHV0LnRzIl0sInNvdXJjZXNDb250ZW50IjpbImltcG9ydCAqIGFzIGVtb3Rpb25SZWFjdCBmcm9tIFwiQGVtb3Rpb24vcmVhY3RcIjtcbmltcG9ydCB7IFB1cmVDb21wb25lbnQgfSBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCBSZWFjdERPTSBmcm9tIFwicmVhY3QtZG9tXCI7XG5cbmNvbnN0IHN0eWxlc0luQ2FsbGJhY2sgPSAocHJvcHM6IGFueSkgPT5cbiAgZW1vdGlvblJlYWN0LmNzcyh7XG4gICAgY29sb3I6IFwicmVkXCIsXG4gICAgYmFja2dyb3VuZDogXCJ5ZWxsb3dcIixcbiAgICB3aWR0aDogYCR7cHJvcHMuc2NhbGUgKiAxMDB9cHhgLFxuICB9KTtcblxuY29uc3Qgc3R5bGVzID0gZW1vdGlvblJlYWN0LmNzcyh7XG4gIGNvbG9yOiBcInJlZFwiLFxuICB3aWR0aDogXCIyMHB4XCIsXG59KTtcblxuY29uc3Qgc3R5bGVzMiA9IGVtb3Rpb25SZWFjdC5jc3NgXG4gIGNvbG9yOiByZWQ7XG4gIHdpZHRoOiAyMHB4O1xuYDtcblxuZXhwb3J0IGNsYXNzIFNpbXBsZUNvbXBvbmVudCBleHRlbmRzIFB1cmVDb21wb25lbnQge1xuICByZW5kZXIoKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDw+XG4gICAgICAgIDxkaXYgY2xhc3NOYW1lPXtzdHlsZXN9PlxuICAgICAgICAgIDxzcGFuPmhlbGxvPC9zcGFuPlxuICAgICAgICA8L2Rpdj5cbiAgICAgICAgPGRpdiBjc3M9e3sgY29sb3I6IFwiYmx1ZVwiIH19PmNzcyBwcm9wPC9kaXY+XG4gICAgICA8Lz5cbiAgICApO1xuICB9XG59XG5cblJlYWN0RE9NLnJlbmRlcig8U2ltcGxlQ29tcG9uZW50IC8+LCBkb2N1bWVudC5xdWVyeVNlbGVjdG9yKFwiI2FwcFwiKSk7XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBZ0JnQiJ9 */");
export class SimpleComponent extends PureComponent {
    render() {
        return /*#__PURE__*/ _jsxs(_Fragment, {
            children: [
                /*#__PURE__*/ _jsx("div", {
                    className: styles,
                    children: /*#__PURE__*/ _jsx("span", {
                        children: "hello"
                    })
                }),
                /*#__PURE__*/ _jsx("div", {
                    css: /*#__PURE__*/ emotionReact.css({
                        color: "blue"
                    }, "label:SimpleComponent", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaW5wdXQudHMiLCJzb3VyY2VzIjpbImlucHV0LnRzIl0sInNvdXJjZXNDb250ZW50IjpbImltcG9ydCAqIGFzIGVtb3Rpb25SZWFjdCBmcm9tIFwiQGVtb3Rpb24vcmVhY3RcIjtcbmltcG9ydCB7IFB1cmVDb21wb25lbnQgfSBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCBSZWFjdERPTSBmcm9tIFwicmVhY3QtZG9tXCI7XG5cbmNvbnN0IHN0eWxlc0luQ2FsbGJhY2sgPSAocHJvcHM6IGFueSkgPT5cbiAgZW1vdGlvblJlYWN0LmNzcyh7XG4gICAgY29sb3I6IFwicmVkXCIsXG4gICAgYmFja2dyb3VuZDogXCJ5ZWxsb3dcIixcbiAgICB3aWR0aDogYCR7cHJvcHMuc2NhbGUgKiAxMDB9cHhgLFxuICB9KTtcblxuY29uc3Qgc3R5bGVzID0gZW1vdGlvblJlYWN0LmNzcyh7XG4gIGNvbG9yOiBcInJlZFwiLFxuICB3aWR0aDogXCIyMHB4XCIsXG59KTtcblxuY29uc3Qgc3R5bGVzMiA9IGVtb3Rpb25SZWFjdC5jc3NgXG4gIGNvbG9yOiByZWQ7XG4gIHdpZHRoOiAyMHB4O1xuYDtcblxuZXhwb3J0IGNsYXNzIFNpbXBsZUNvbXBvbmVudCBleHRlbmRzIFB1cmVDb21wb25lbnQge1xuICByZW5kZXIoKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDw+XG4gICAgICAgIDxkaXYgY2xhc3NOYW1lPXtzdHlsZXN9PlxuICAgICAgICAgIDxzcGFuPmhlbGxvPC9zcGFuPlxuICAgICAgICA8L2Rpdj5cbiAgICAgICAgPGRpdiBjc3M9e3sgY29sb3I6IFwiYmx1ZVwiIH19PmNzcyBwcm9wPC9kaXY+XG4gICAgICA8Lz5cbiAgICApO1xuICB9XG59XG5cblJlYWN0RE9NLnJlbmRlcig8U2ltcGxlQ29tcG9uZW50IC8+LCBkb2N1bWVudC5xdWVyeVNlbGVjdG9yKFwiI2FwcFwiKSk7XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBNEJrQiJ9 */"),
                    children: "css prop"
                })
            ]
        });
    }
}
ReactDOM.render(/*#__PURE__*/ _jsx(SimpleComponent, {}), document.querySelector("#app"));
