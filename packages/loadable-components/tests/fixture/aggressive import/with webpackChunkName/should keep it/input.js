import loadable from "@loadable/component";
loadable(
  (props) =>
    import(/* webpackChunkName: "pages/[request]" */ `./pages/${props.path}`),
);
