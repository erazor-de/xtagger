use crate::args::Args;
use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use xtag::Searcher;

pub struct App {
    pub args: Args,
    pub filter: Option<Searcher>,
}

impl App {
    pub fn new() -> Result<Self> {
        let args = Args::parse();
        custom_validation(&args)?;
        let filter = get_searcher(&args)?;
        Ok(App { args, filter })
    }
}

fn get_searcher(args: &Args) -> Result<Option<Searcher>> {
    let filter = if let Some(term) = &args.filter.filter {
        Some(term.to_owned())
    } else if let Some(link) = &args.filter.bookmark {
        Some(load_bookmark(link)?)
    } else {
        None
    };
    filter.map_or(Ok(None), |term| Ok(Some(xtag::compile_search(&term)?)))
}

fn custom_validation(_args: &Args) -> Result<()> {
    // FIXME add sanity check for rename:
    // if find expression has capture group, replace expression needs $
    // maybe escaping has been forgotten
    Ok(())
}

// Bookmark is symbolic link with the filter term as link
fn load_bookmark(path: &PathBuf) -> Result<String> {
    fs::read_link(path)?
        .into_os_string()
        .into_string()
        .map_err(|os_string| anyhow!("'{:?}' is no valid filter term", os_string))
}
