import { styled } from "@example/example";
const Test = styled.div.withConfig({
    displayName: "Test",
    componentId: "sc-7d0c827c-0"
})`width:100%;`;
const Test2 = true ? styled.div.withConfig({
    displayName: "Test2",
    componentId: "sc-7d0c827c-1"
})`` : styled.div.withConfig({
    displayName: "Test2",
    componentId: "sc-7d0c827c-2"
})``;
const styles = {
    One: styled.div.withConfig({
        displayName: "One",
        componentId: "sc-7d0c827c-3"
    })``
};
let Component;
Component = styled.div.withConfig({
    displayName: "Component",
    componentId: "sc-7d0c827c-4"
})``;
const WrappedComponent = styled(Inner).withConfig({
    displayName: "WrappedComponent",
    componentId: "sc-7d0c827c-5"
})``;
