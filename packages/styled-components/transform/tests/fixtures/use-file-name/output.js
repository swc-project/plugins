import styled from "styled-components";
const Test = styled.div.withConfig({
    displayName: "code__Test",
    componentId: "sc-da71f793-0"
})`color:red;`;
const before = styled.div.withConfig({
    displayName: "code__before",
    componentId: "sc-da71f793-1"
})`color:blue;`;
styled.div.withConfig({
    displayName: "code",
    componentId: "sc-da71f793-2"
})``;
export default styled.button.withConfig({
    displayName: "code",
    componentId: "sc-da71f793-3"
})``;
