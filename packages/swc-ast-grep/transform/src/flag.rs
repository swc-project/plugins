use swc_common::{comments::Comments, errors::HANDLER, Spanned};
use swc_ecma_ast::{
    Expr, KeyValueProp, ObjectLit, Pat, Prop, PropName, PropOrSpread, VarDeclarator,
};
use swc_ecma_utils::ExprFactory;

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
        let init = v.init.as_deref_mut()?;
        let call_expr = init.as_mut_call()?;

        let callee = call_expr.callee.as_mut_expr()?;

        let import_of_flag_callee = self
            .imports
            .is_in_import_items(callee, &self.config.flag.import_sources)?;

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
                            .span_note(import_of_flag_callee, "flag() is imported here")
                            .emit();
                    });
                }
                return None;
            }
        };

        let prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident("key".into()),
            value: name.sym.clone().into(),
        })));

        if call_expr.args.is_empty() {
            call_expr.args.push(
                ObjectLit {
                    props: vec![prop],
                    ..Default::default()
                }
                .as_arg(),
            );
        } else if let Some(obj) = find_object(&mut call_expr.args[0].expr) {
            if obj
                .props
                .iter()
                .filter_map(|p| p.as_prop())
                .any(|p| match &**p {
                    Prop::KeyValue(KeyValueProp { key, .. }) => {
                        matches!(key, PropName::Ident(i) if i.sym == "key")
                    }
                    _ => false,
                })
            {
                return None;
            }

            obj.props.push(prop);
        }

        None
    }
}

fn find_object(arg: &mut Expr) -> Option<&mut ObjectLit> {
    match arg {
        Expr::Object(obj) => Some(obj),
        Expr::Call(call) => {
            if call.args.is_empty() {
                call.args.push(
                    ObjectLit {
                        ..Default::default()
                    }
                    .as_arg(),
                );
            }

            let arg = call.args.get_mut(0)?;
            find_object(&mut arg.expr)
        }
        _ => None,
    }
}
