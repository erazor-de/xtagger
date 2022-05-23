use clap::{ArgGroup, Parser};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("printing").multiple(false).required(false).args(&["list", "show", "tags"])))]
#[clap(group(ArgGroup::new("manipulating").multiple(true).required(false).args(&["add", "remove", "find", "replace"])))]
#[clap(group(ArgGroup::new("deleting").multiple(false).required(false).args(&["delete"]).conflicts_with("manipulation")))]
#[clap(group(ArgGroup::new("renaming").multiple(true).required(false).args(&["find", "replace"]).requires_all(&["find", "replace"])))]
#[clap(group(ArgGroup::new("filtering").multiple(false).required(false).args(&["filter", "bookmark"])))]
pub struct Args {
    /// Print files as hyperlinks
    #[clap(short, long)]
    pub hyperlink: bool,

    // Manipulation options
    /// Don't change anything
    #[clap(short, long)]
    pub dry_run: bool,

    #[clap(flatten)]
    pub print: Print,

    #[clap(flatten)]
    pub manipulate: Manipulate,

    #[clap(flatten)]
    pub filter: Filter,

    // Args
    #[clap(parse(from_os_str), value_name = "PATH")]
    pub paths: Vec<PathBuf>,
}

#[derive(clap::Args)]
pub struct Manipulate {
    /// Add tags
    #[clap(short, long, value_name = "TAGS", parse(try_from_str=xtag::csl_to_map))]
    pub add: Option<HashMap<String, Option<String>>>,

    /// Remove tags
    #[clap(short, long, value_name = "TAGS", parse(try_from_str=xtag::csl_to_map))]
    pub remove: Option<HashMap<String, Option<String>>>,

    /// Delete tags
    #[clap(long)]
    pub delete: bool,

    #[clap(flatten)]
    pub rename: Rename,
}

#[derive(clap::Args)]
pub struct Filter {
    /// filter per search term
    #[clap(short, long, value_name = "TERM")]
    pub filter: Option<String>,

    /// filter per bookmark
    #[clap(short, long, value_name = "PATH", parse(from_os_str))]
    pub bookmark: Option<PathBuf>,
}

#[derive(clap::Args)]
pub struct Rename {
    #[clap(long, value_name = "REGEX")]
    pub find: Option<String>,

    #[clap(long, value_name = "REGEX")]
    pub replace: Option<String>,
}

#[derive(clap::Args)]
pub struct Print {
    /// List files
    #[clap(short, long)]
    pub list: bool,

    /// List files with tags and values
    #[clap(short, long)]
    pub show: bool,

    /// List all used tags
    #[clap(short, long)]
    pub tags: bool,
}
