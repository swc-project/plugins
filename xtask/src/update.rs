use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use clap::Args;

#[derive(Debug, Args)]
pub struct Update {}

impl Update {
    pub fn run(self) -> Result<()> {
        update_swc_crates().context("failed to update swc crates")?;

        Ok(())
    }
}

fn update_swc_crates() -> Result<()> {
    let md = MetadataCommand::new().exec()?;

    dbg!(&md);

    Ok(())
}
