use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};
#[cfg(feature = "parallel")]
use rayon::prelude::*;
use regex::Regex;
use swc_common::{
    collections::{AHashMap, AHashSet},
    sync::Lazy,
};
use swc_core::css::ast::ListOfComponentValues;

/// Content of the config file
#[derive(Debug)]
pub struct Config {
    content: Vec<String>,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self> {
        todo!()
    }
}

static CONTENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"['"\s<>=/]"#).unwrap());

#[derive(Debug)]
pub struct Tailwind {
    config_path: PathBuf,
}

impl Tailwind {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    pub fn compile(&mut self) -> Result<()> {
        let config = Config::from_path(&self.config_path)
            .context("failed to load config file")
            .map(Arc::new)?;

        let files = resolve_glob(&config.content);

        // Collect candidates, optionally in parallel
        #[cfg(not(feature = "parallel"))]
        let iter = files.into_iter();

        #[cfg(feature = "parallel")]
        let iter = files.into_par_iter();

        // TODO: Optimize collect out
        let candidates = iter
            .map(|f| {
                let contents = read_to_string(&f).context("failed to read file")?;

                let s = CONTENT_RE
                    .split(&contents)
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();

                Ok(s)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<AHashSet<_>>();

        let mut plugins: Vec<Plugin> = vec![];

        // Built-in plugins
        plugins.push(Box::new(|context| {
            let mut map = AHashMap::default();

            map.insert(".built-in-utility".into(), {
                let mut m = AHashMap::default();
                m.insert("color".into(), "red".into());
                m
            });
            map.insert(".should-not-be-generated".into(), {
                let mut m = AHashMap::default();
                m.insert("appearance".into(), "none".into());
                m
            });

            context.add_utilities(map);
        }));

        // Example built-in plugin that can read values from the config.

        plugins.push({
            let config = config.clone();
            Box::new(move |context| {
                let map = AHashMap::default();

                // TODO: Convert config to a hash map

                // This is an example of using config from core plugins
                #[allow(clippy::drop_ref)]
                drop(&config);
                context.add_utilities(map);
            })
        });

        Ok(())
    }
}

fn resolve_glob(config: &[String]) -> Vec<PathBuf> {
    todo!()
}

type Plugin = Box<dyn Fn(&mut PluginContext)>;

pub struct PluginContext {}

impl PluginContext {
    pub fn add_utilities(&mut self, map: AHashMap<String, AHashMap<String, String>>) {}
}
