import styled from "@xstyled/styled-components";
const Test = styled.div.withConfig({
    componentId: "sc-347ea48d-0"
})`width:100%;`;
const Test2 = true ? styled.div.withConfig({
    componentId: "sc-347ea48d-1"
})`` : styled.div.withConfig({
    componentId: "sc-347ea48d-2"
})``;
const styles = {
    One: styled.div.withConfig({
        componentId: "sc-347ea48d-3"
    })``
};
let Component;
Component = styled.div.withConfig({
    componentId: "sc-347ea48d-4"
})``;
const WrappedComponent = styled(Inner).withConfig({
    componentId: "sc-347ea48d-5"
})``;
