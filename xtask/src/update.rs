use std::process::Command;

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

/// Get the list of swc crates in the main swc repository.
fn get_swc_crates() -> Result<Vec<String>> {
    let md = MetadataCommand::new().exec()?;

    Ok(md
        .packages
        .iter()
        .filter(|p| p.repository.as_deref() == Some("https://github.com/swc-project/swc.git"))
        .map(|p| p.name.clone())
        .collect::<Vec<_>>())
}

fn update_swc_crates() -> Result<()> {
    let mut c = Command::new("cargo");
    c.arg("upgrade")
        .arg("--incompatible")
        .arg("--recursive")
        .arg("false");

    for pkg in get_swc_crates()? {
        c.arg("--package").arg(pkg);
    }

    c.status()?;

    Ok(())
}
