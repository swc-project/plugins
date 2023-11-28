import styled from "styled-components";
const Test = styled.div.withConfig({
    displayName: "use-directory-name__Test",
    componentId: "sc-a2611da4-0"
})`color:red;`;
const before = styled.div.withConfig({
    displayName: "use-directory-name__before",
    componentId: "sc-a2611da4-1"
})`color:blue;`;
styled.div.withConfig({
    displayName: "use-directory-name",
    componentId: "sc-a2611da4-2"
})``;
export default styled.button.withConfig({
    displayName: "use-directory-name",
    componentId: "sc-a2611da4-3"
})``;
