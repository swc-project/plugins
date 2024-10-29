use serde::Deserialize;
use swc_cached::regex::CachedRegex;
use swc_ecma_ast::*;
use swc_ecma_visit::{fold_pass, noop_fold_type, Fold, FoldWith};

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Config {
    All(bool),
    WithOptions(Options),
}

impl Config {
    pub fn truthy(&self) -> bool {
        match self {
            Config::All(b) => *b,
            Config::WithOptions(_) => true,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Options {
    #[serde(default)]
    pub properties: Vec<String>,
}

pub fn react_remove_properties(config: Config) -> impl Pass {
    let mut properties: Vec<CachedRegex> = match config {
        Config::WithOptions(x) => x
            .properties
            .iter()
            .map(|pattern| {
                CachedRegex::new(pattern).unwrap_or_else(|e| {
                    panic!("error compiling property regex `{}`: {}", pattern, e);
                })
            })
            .collect(),
        _ => vec![],
    };
    if properties.is_empty() {
        // Keep the default regex identical to `babel-plugin-react-remove-properties`.
        properties.push(CachedRegex::new(r"^data-test").unwrap());
    }
    fold_pass(RemoveProperties { properties })
}

struct RemoveProperties {
    properties: Vec<CachedRegex>,
}

impl RemoveProperties {
    fn should_remove_property(&self, name: &str) -> bool {
        self.properties.iter().any(|p| p.is_match(name))
    }
}

impl Fold for RemoveProperties {
    noop_fold_type!();

    fn fold_jsx_opening_element(&mut self, mut el: JSXOpeningElement) -> JSXOpeningElement {
        el.attrs.retain(|attr| {
            !matches!(attr, JSXAttrOrSpread::JSXAttr(JSXAttr {
              name: JSXAttrName::Ident(ident),
              ..
            }) if self.should_remove_property(ident.sym.as_ref()))
        });
        el.fold_children_with(self)
    }
}
