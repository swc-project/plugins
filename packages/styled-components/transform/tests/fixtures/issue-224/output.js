import { styled } from "styled-components";
const Test = styled.div.withConfig({
    displayName: "code__Test",
    componentId: "sc-d7113d7e-0"
})([
    `width:100%;`
]);
const Test2 = true ? styled.div.withConfig({
    displayName: "code__Test2",
    componentId: "sc-d7113d7e-1"
})([
    ``
]) : styled.div.withConfig({
    displayName: "code__Test2",
    componentId: "sc-d7113d7e-2"
})([
    ``
]);
const styles = {
    One: styled.div.withConfig({
        displayName: "code__One",
        componentId: "sc-d7113d7e-3"
    })([
        ``
    ])
};
let Component;
Component = styled.div.withConfig({
    displayName: "code__Component",
    componentId: "sc-d7113d7e-4"
})([
    ``
]);
const WrappedComponent = styled(Inner).withConfig({
    displayName: "code__WrappedComponent",
    componentId: "sc-d7113d7e-5"
})([
    ``
]);
