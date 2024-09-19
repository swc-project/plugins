use swc_common::comments::Comments;
use swc_ecma_ast::VarDeclarator;

use crate::SwcSdkTransform;

impl<C> SwcSdkTransform<C>
where
    C: Comments,
{
    pub(super) fn transform_flag(&mut self, v: &mut VarDeclarator) {}
}
