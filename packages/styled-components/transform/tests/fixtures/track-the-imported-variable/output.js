import s from "styled-components";
const Test = s.div.withConfig({
  displayName: "Test",
  componentId: "sc-e3f2753-0",
})`width:100%;`;
const Test2 = true
  ? s.div.withConfig({
      displayName: "Test2",
      componentId: "sc-e3f2753-1",
    })``
  : s.div.withConfig({
      displayName: "Test2",
      componentId: "sc-e3f2753-2",
    })``;
const styles = {
  One: s.div.withConfig({
    displayName: "One",
    componentId: "sc-e3f2753-3",
  })``,
};
let Component;
Component = s.div.withConfig({
  displayName: "Component",
  componentId: "sc-e3f2753-4",
})``;
const WrappedComponent = s(Inner).withConfig({
  displayName: "WrappedComponent",
  componentId: "sc-e3f2753-5",
})``;
