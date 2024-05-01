import loadable from "@loadable/component";
loadable((props) => import(/* webpackChunkName: "Pages" */ `./${props.foo}`));
