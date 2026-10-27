import loadable from "@loadable/component";
loadable(() => import("./OtherComponent"), { ssr: false });
