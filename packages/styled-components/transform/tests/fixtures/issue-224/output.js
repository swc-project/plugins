import { styled } from 'styled-components';
const Test = styled.div.withConfig({
    displayName: "code__Test",
    componentId: "sc-8043c6cb-0"
})`
  width: 100%;
`;
const Test2 = true ? styled.div.withConfig({
    displayName: "code__Test2",
    componentId: "sc-8043c6cb-1"
})`` : styled.div.withConfig({
    displayName: "code__Test2",
    componentId: "sc-8043c6cb-2"
})``;
const styles = {
    One: styled.div.withConfig({
        displayName: "code__One",
        componentId: "sc-8043c6cb-3"
    })``
};
let Component;
Component = styled.div.withConfig({
    displayName: "code__Component",
    componentId: "sc-8043c6cb-4"
})``;
const WrappedComponent = styled(Inner).withConfig({
    displayName: "code__WrappedComponent",
    componentId: "sc-8043c6cb-5"
})``;
