import loadable from "@loadable/component";
loadable(() => timeout(import("./ModA"), 2000));
