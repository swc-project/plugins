declare module "@swc/plugin-relay" {
  export interface Config {
    rootDir: string;
    artifactDirectory?: string;
    language: "typescript" | "javascrip" | "flow";
    eagerEsModules?: boolean;
  }
}
