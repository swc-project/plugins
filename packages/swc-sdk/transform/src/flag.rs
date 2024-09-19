use swc_common::comments::Comments;
use swc_ecma_ast::VarDeclarator;

use crate::SwcSdkTransform;

impl<C> SwcSdkTransform<C>
where
    C: Comments,
{
    pub(super) fn transform_flag(&mut self, v: &mut VarDeclarator) {
        let is_flag_call = v.init.as_deref().map_or(false, |e| {
            self.imports
                .is_in_import_items(e, &self.config.flag.import_sources)
        });

        if !is_flag_call {
            return;
        }
    }
}
