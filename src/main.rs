use anyhow::Result;
use clap::Parser;
use xtagger::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    xtagger::custom_validation(&args);

    xtagger::run(&args)
}
