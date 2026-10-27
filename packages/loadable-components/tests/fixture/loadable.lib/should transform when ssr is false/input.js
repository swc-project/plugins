import loadable from "@loadable/component";
loadable.lib(() => import("./OtherComponent"), { ssr: false });
