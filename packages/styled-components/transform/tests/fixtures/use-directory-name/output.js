import styled from "styled-components";
const Test = styled.div.withConfig({
    displayName: "use-directory-name__Test",
    componentId: "sc-da71f793-0"
})`color:red;`;
const before = styled.div.withConfig({
    displayName: "use-directory-name__before",
    componentId: "sc-da71f793-1"
})`color:blue;`;
styled.div.withConfig({
    displayName: "use-directory-name",
    componentId: "sc-da71f793-2"
})``;
export default styled.button.withConfig({
    displayName: "use-directory-name",
    componentId: "sc-da71f793-3"
})``;
