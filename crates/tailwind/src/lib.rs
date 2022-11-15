use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
#[cfg(feature = "parallel")]
use rayon::prelude::*;
use regex::Regex;
use swc_common::{collections::AHashSet, sync::Lazy};

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
        let config = Config::from_path(&self.config_path).context("failed to load config file")?;

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

        Ok(())
    }
}

fn resolve_glob(config: &[String]) -> Vec<PathBuf> {
    todo!()
}

enum Plugin {}
