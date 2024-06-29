import loadable from "@loadable/component";
loadable(
  () =>
    import(/* webpackPrefetch: true, webpackChunkName: "ChunkA" */ "./ModA"),
);
