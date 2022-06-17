declare module "@swc/plugin-relay" {
  export interface Config {
    rootDir: string;
    artifactDirectory?: string;
    module: "commonjs" | "esm";
    language: "typescript" | "flow";
  }
}
