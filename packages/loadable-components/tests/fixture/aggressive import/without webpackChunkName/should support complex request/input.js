import loadable from "@loadable/component";
loadable((props) => import(`./dir/${props.foo}/test`));
