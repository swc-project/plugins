import styled from "styled-components";
const Test = /*#__PURE__*/ styled.div.withConfig({
    displayName: "code__Test",
    componentId: "sc-eb129dbe-0"
})([
    `width:100%;`
]);
const Test2 = /*#__PURE__*/ styled("div").withConfig({
    displayName: "code__Test2",
    componentId: "sc-eb129dbe-1"
})([
    ``
]);
const Test3 = true ? /*#__PURE__*/ styled.div.withConfig({
    displayName: "code__Test3",
    componentId: "sc-eb129dbe-2"
})([
    ``
]) : /*#__PURE__*/ styled.div.withConfig({
    displayName: "code__Test3",
    componentId: "sc-eb129dbe-3"
})([
    ``
]);
const styles = {
    One: /*#__PURE__*/ styled.div.withConfig({
        displayName: "code__One",
        componentId: "sc-eb129dbe-4"
    })([
        ``
    ])
};
let Component;
Component = /*#__PURE__*/ styled.div.withConfig({
    displayName: "code__Component",
    componentId: "sc-eb129dbe-5"
})([
    ``
]);
const WrappedComponent = /*#__PURE__*/ styled(Inner).withConfig({
    displayName: "code__WrappedComponent",
    componentId: "sc-eb129dbe-6"
})([
    ``
]);
const StyledObjectForm = /*#__PURE__*/ styled.div.withConfig({
    displayName: "code__StyledObjectForm",
    componentId: "sc-eb129dbe-7"
})({
    color: red
});
const StyledFunctionForm = /*#__PURE__*/ styled.div.withConfig({
    displayName: "code__StyledFunctionForm",
    componentId: "sc-eb129dbe-8"
})((p)=>({
        color: p.color || "red"
    }));
const normalFunc = add(5, 3);
