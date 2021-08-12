#![crate_type = "cdylib"]

use abi_stable::export_root_module;
use abi_stable::prefix_type::PrefixTypeTrait;
use abi_stable::sabi_extern_fn;
use abi_stable::std_types::RResult;
use abi_stable::std_types::RStr;
use abi_stable::std_types::RString;
use swc_plugin_shared::SwcPlugin;
use swc_plugin_shared::SwcPluginRef;
use RResult::ROk;

#[export_root_module]
pub fn get_library() -> SwcPluginRef {
    SwcPlugin {
        get_js_ast_version,
        process_js,
    }
    .leak_into_prefix()
}

#[sabi_extern_fn]
fn get_js_ast_version() -> RString {
    "0.1.0".into()
}

#[sabi_extern_fn]
fn process_js(_config_json: RStr, ast_json: RStr) -> RResult<RString, RString> {
    ROk(ast_json.into())
}
