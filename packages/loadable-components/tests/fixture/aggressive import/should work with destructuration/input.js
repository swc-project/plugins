import loadable from "@loadable/component";
loadable(({ foo }) => import(/* webpackChunkName: "Pages" */ `./${foo}`));
