import styled from "styled-components";
const Test = styled.div.withConfig({
    componentId: "sc-6f6ed417-0"
})`width:100%;`;
const Test2 = true ? styled.div.withConfig({
    componentId: "sc-6f6ed417-1"
})`` : styled.div.withConfig({
    componentId: "sc-6f6ed417-2"
})``;
const styles = {
    One: styled.div.withConfig({
        componentId: "sc-6f6ed417-3"
    })``
};
let Component;
Component = styled.div.withConfig({
    componentId: "sc-6f6ed417-4"
})``;
const WrappedComponent = styled(Inner).withConfig({
    componentId: "sc-6f6ed417-5"
})``;
