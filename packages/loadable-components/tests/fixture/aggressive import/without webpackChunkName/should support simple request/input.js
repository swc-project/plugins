import loadable from "@loadable/component";
loadable((props) => import(`./${props.foo}`));
