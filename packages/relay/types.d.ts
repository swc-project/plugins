declare module "@swc/plugin-relay" {
  export interface Config {
    rootDir: string;
    artifactDirectory?: string;
    language: "typescript" | "javascript" | "flow";
    eagerEsModules?: boolean;
  }
}
