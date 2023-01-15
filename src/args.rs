use std::path::PathBuf;

use clap::{ArgAction, ArgGroup, Args, Parser};
use xtag::XTags;

// FIXME print help if no args
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
// #[command(group(ArgGroup::new("any").multiple(true).required(true).
//     args(&["add", "remove", "find", "replace", "copy", "delete", "find", "replace", "filter", "bookmark"])))]
#[command(group(ArgGroup::new("printing").multiple(false).required(false).
    args(&["list", "show", "tags"])))]
#[command(group(ArgGroup::new("manipulating").multiple(true).required(false).
    args(&["add", "remove", "copy", "find", "replace"])))]
#[command(group(ArgGroup::new("deleting").multiple(false).required(false).
    args(&["delete"]).
    conflicts_with("manipulating").
    conflicts_with("renaming")))]
#[command(group(ArgGroup::new("renaming").multiple(true).required(false).
    args(&["find", "replace"]).
    requires_all(&["find", "replace"])))]
#[command(group(ArgGroup::new("filtering").multiple(false).required(false).
    args(&["filter", "bookmark"])))]
#[command(disable_help_flag = true)]
pub struct Arguments {
    // Use only long version of help, because short clashes with hyperlink.
    // Don't know why Option is needed to make argument optional in this case.
    /// Print help information
    #[arg(long, action = ArgAction::Help)]
    _help: Option<bool>,

    /// Print files as hyperlinks
    #[arg(short, long)]
    pub hyperlink: bool,

    /// Don't change anything
    #[arg(short, long)]
    pub dry_run: bool,

    #[command(flatten)]
    pub print: Print,

    #[command(flatten)]
    pub manipulate: Manipulate,

    #[command(flatten)]
    pub filter: Filter,

    // Args
    #[arg(value_name = "GLOB", value_hint = clap::ValueHint::DirPath)]
    pub globs: Vec<PathBuf>,
}

#[derive(Args)]
pub struct Manipulate {
    /// Add tags
    #[arg(short, long, value_name = "TAGS", value_parser = xtag::csl_to_map)]
    pub add: Option<XTags>,

    /// Remove tags
    #[arg(short, long, value_name = "TAGS", value_parser = xtag::csl_to_map)]
    pub remove: Option<XTags>,

    /// Delete tags
    #[arg(long)]
    pub delete: bool,

    /// Copy tags
    #[arg(long)]
    pub copy: bool,

    #[command(flatten)]
    pub rename: Rename,
}

#[derive(Args)]
pub struct Filter {
    /// filter per search term
    #[arg(short, long, value_name = "TERM")]
    pub filter: Option<String>,

    /// filter per bookmark
    #[arg(short, long, value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    pub bookmark: Option<PathBuf>,
}

#[derive(Args)]
pub struct Rename {
    #[arg(long, value_name = "REGEX")]
    pub find: Option<String>,

    #[arg(long, value_name = "REGEX")]
    pub replace: Option<String>,
}

#[derive(Args)]
pub struct Print {
    /// List files
    #[arg(short, long)]
    pub list: bool,

    /// List files with tags and values
    #[arg(short, long)]
    pub show: bool,

    /// List all used tags
    #[arg(short, long)]
    pub tags: bool,
}
