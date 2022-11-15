use std::path::{Path, PathBuf};

use anyhow::Result;

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

#[derive(Debug)]
pub struct Tailwind {
    config_path: PathBuf,
}

impl Tailwind {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    pub fn compile(&mut self) -> Result<()> {
        let config = Config::from_path(&self.config_path)?;

        Ok(())
    }
}
