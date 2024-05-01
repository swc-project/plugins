import loadable from "@loadable/component";
loadable(({ foo }) => import(`./dir/${foo}/test`));
