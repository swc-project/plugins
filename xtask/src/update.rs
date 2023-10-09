use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct Update {}

impl Update {
    pub fn run(self) -> Result<()> {}
}
