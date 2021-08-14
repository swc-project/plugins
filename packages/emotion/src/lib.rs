use abi_stable::export_root_module;
use abi_stable::prefix_type::PrefixTypeTrait;
use abi_stable::sabi_extern_fn;
use abi_stable::std_types::RResult;
use abi_stable::std_types::RStr;
use abi_stable::std_types::RString;
use swc_plugin::ecmascript::ast::ModuleItem;
use swc_plugin::ecmascript::ast::Program;
use swc_plugin::ecmascript::visit::noop_fold_type;
use swc_plugin::ecmascript::visit::Fold;
use swc_plugin::ecmascript::visit::FoldWith;
use swc_plugin::SwcPlugin;
use swc_plugin::SwcPluginRef;
use RResult::ROk;

#[export_root_module]
pub fn get_library() -> SwcPluginRef {
    SwcPlugin {
        get_js_ast_version,
        process_js: Some(process_js),
    }
    .leak_into_prefix()
}

#[sabi_extern_fn]
fn get_js_ast_version() -> RString {
    "0.1.0".into()
}

#[sabi_extern_fn]
fn process_js(_config_json: RStr, ast_json: RString) -> RResult<RString, RString> {
    let ast: Program = serde_json::from_slice(ast_json.as_bytes()).unwrap();

    let ast = ast.fold_with(&mut Transform {});

    let ast_json = serde_json::to_string(&ast).unwrap();
    ROk(ast_json.into())
}

struct Transform {}

impl Fold for Transform {
    noop_fold_type!();

    // Drop everything
    fn fold_module_items(&mut self, _: Vec<ModuleItem>) -> Vec<ModuleItem> {
        Vec::new()
    }
}
