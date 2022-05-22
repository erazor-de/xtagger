use clap::{ArgGroup, Parser};
use std::collections::HashMap;
use std::path::PathBuf;
use xtag::Searcher;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("output").multiple(false).required(false).args(&["list", "show", "tags"])))]
#[clap(group(ArgGroup::new("modification").multiple(true).required(false).args(&["add", "remove", "find"])))]
#[clap(group(ArgGroup::new("manipulation").multiple(false).required(false).args(&["delete"]).conflicts_with("modification")))]
#[clap(group(ArgGroup::new("rename").multiple(true).required(false).args(&["find", "replace"]).requires_all(&["find", "replace"])))]
pub struct Args {
    /// Print files as hyperlinks
    #[clap(short, long)]
    pub hyperlink: bool,

    /// filter per search term
    #[clap(short, long, value_name = "TERM", parse(try_from_str=xtag::compile_search))]
    pub filter: Option<Searcher>,

    // Manipulation options
    /// Add tags
    #[clap(short, long, value_name = "TAGS", parse(try_from_str=xtag::csl_to_map))]
    pub add: Option<HashMap<String, Option<String>>>,

    /// Remove tags
    #[clap(short, long, value_name = "TAGS", parse(try_from_str=xtag::csl_to_map))]
    pub remove: Option<HashMap<String, Option<String>>>,

    /// Rename tags
    #[clap(long, value_name = "REGEX")]
    pub find: Option<String>,

    /// Rename tags
    #[clap(long, value_name = "REGEX")]
    pub replace: Option<String>,

    /// Don't change anything
    #[clap(short, long)]
    pub dry_run: bool,

    /// Delete tags
    #[clap(long)]
    pub delete: bool,

    // Output options, only one allowed and needed
    /// List files
    #[clap(short, long)]
    pub list: bool,

    /// List files with tags and values
    #[clap(short, long)]
    pub show: bool,

    /// List all used tags
    #[clap(short, long)]
    pub tags: bool,

    // Args
    #[clap(parse(from_os_str), value_name = "PATH")]
    pub paths: Vec<PathBuf>,
}

pub fn custom_validation(_args: &Args) -> bool {
    // FIXME add sanity check for rename:
    // if find expression has capture group, replace expression needs $
    // maybe escaping has been forgotten
    true
}
