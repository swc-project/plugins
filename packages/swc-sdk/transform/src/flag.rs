use swc_common::{comments::Comments, errors::HANDLER, Spanned};
use swc_ecma_ast::{Pat, VarDeclarator};

use crate::SwcSdkTransform;

impl<C> SwcSdkTransform<C>
where
    C: Comments,
{
    ///
    /// ## Cases
    ///
    /// ### Empty arugments
    ///
    /// ```js
    /// 
    /// import { flag } from "@swc/sdk/flag";
    ///
    /// const foo = flag();
    /// ```
    ///
    /// becomes
    ///
    /// ```js
    /// import { flag } from "@swc/sdk/flag";
    ///
    /// const foo = flag({
    ///   key: "foo",
    /// });
    /// ```
    ///
    /// ### With arguments
    ///
    /// ```js
    /// import { flag } from "@swc/sdk/flag";
    ///
    /// const foo = flag({
    ///     decide: () => false,
    /// });
    /// ```
    ///
    /// becomes
    ///
    /// ```js
    /// import { flag } from "@swc/sdk/flag";
    ///
    /// const foo = flag({
    ///     key: "foo",
    ///     decide: () => false,
    /// });
    /// ```
    ///
    /// ### With custom adapter
    ///
    ///
    /// ```js
    /// import { flag } from "@swc/sdk/flag";
    ///
    /// const foo = flag(someAdapter({
    ///     decide: () => false,
    /// });
    /// ```
    ///
    /// becomes
    ///
    /// ```js
    /// import { flag } from "@swc/sdk/flag";
    ///
    /// const foo = flag(someAdapter({
    ///     key: "foo",
    ///     decide: () => false,
    /// }));
    /// ```
    pub(super) fn transform_flag(&mut self, v: &mut VarDeclarator) -> Option<!> {
        let init = v.init.as_deref()?;
        let is_flag_call = self
            .imports
            .is_in_import_items(init, &self.config.flag.import_sources)?;

        let name = match &v.name {
            Pat::Ident(i) => i.clone(),
            _ => {
                if self.config.flag.strict {
                    HANDLER.with(|handler| {
                        handler
                            .struct_span_err(
                                v.name.span(),
                                "The variable name for the `flag()` calls must be an identifier",
                            )
                            .span_note(is_flag_call, "flag() is imported here")
                            .emit();
                    });
                }
                return None;
            }
        };

        None
    }
}
