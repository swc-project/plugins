loadable(
  () =>
    import(/* webpackPrefetch: true, webpackChunkName: "ChunkA" */ "./ModA"),
);
