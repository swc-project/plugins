#[plugin_transform]
fn noop(mut program: Program, _: TransformPluginProgramMetadata) -> Program {
    program
}
