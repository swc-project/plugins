use swc_common::comments::Comments;
use swc_ecma_ast::VarDeclarator;

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
