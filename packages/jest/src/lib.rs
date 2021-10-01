/// The function which exports the root module of the library.
///
/// The root module is exported inside a static of `LibHeader` type,
/// which has this extra metadata:
///
/// - The abi_stable version number used by the dynamic library.
///
/// - A constant describing the layout of the exported root module,and every
///   type it references.
///
/// - A lazily initialized reference to the root module.
///
/// - The constructor function of the root module.
#[export_root_module]
pub fn get_library() -> SwcPlugin_Ref {
    ExampleLib {
        new_appender,
        new_boxed_interface,
        append_string,
    }
    .leak_into_prefix()
}
