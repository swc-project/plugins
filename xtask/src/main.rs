use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::update::Update;

mod update;
#[derive(Debug, Parser)]

struct CliArgs {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    Update(Update),
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    match args.cmd {
        Cmd::Update(update) => update.run(),
    }
}
