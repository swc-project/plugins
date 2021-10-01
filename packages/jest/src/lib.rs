use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait};
use swc_plugin::{SwcPlugin, SwcPluginRef};

#[export_root_module]
pub fn get_library() -> SwcPluginRef {
    SwcPlugin {}.leak_into_prefix()
}
