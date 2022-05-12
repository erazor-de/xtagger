use clap::{Parser, Subcommand};
use glob::{glob_with, MatchOptions};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;
use std::str;
use xtag::TaggerError;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add tags
    Add {
        /// Comma separated list of tags
        #[clap(value_name = "TAGS")]
        tags: String,

        #[clap(parse(from_os_str), value_name = "GLOB")]
        globs: Vec<PathBuf>,
    },

    /// Remove tags
    Remove {
        /// Comma separated list of tags
        #[clap(value_name = "TAGS")]
        tags: String,

        #[clap(parse(from_os_str), value_name = "GLOB")]
        globs: Vec<PathBuf>,
    },

    /// Delete all tags
    Delete {
        #[clap(parse(from_os_str), value_name = "GLOB")]
        globs: Vec<PathBuf>,
    },

    /// List files with tags and values
    List {
        #[clap(parse(from_os_str), value_name = "GLOB")]
        globs: Vec<PathBuf>,
    },

    /// Find matching files
    Find {
        /// Search term
        #[clap(value_name = "TERM")]
        term: String,

        #[clap(parse(from_os_str), value_name = "GLOB")]
        globs: Vec<PathBuf>,
    },

    /// Rename tags
    Rename {
        #[clap(value_name = "TERM")]
        find: String,

        #[clap(value_name = "TERM")]
        replace: String,

        #[clap(parse(from_os_str), value_name = "GLOB")]
        globs: Vec<PathBuf>,
    },

    /// List all used tags
    Tags {
        #[clap(parse(from_os_str), value_name = "GLOB")]
        globs: Vec<PathBuf>,
    },
}

// Adds tags to given file
// FIXME Prints no messages
fn add(path: &PathBuf, tags: &HashMap<String, Option<String>>) -> Result<(), TaggerError> {
    let mut container = xtag::get_tags(&path)?;
    container.extend(tags.clone());
    xtag::set_tags(&path, &container)?;
    Ok(())
}

fn delete(path: &PathBuf) -> Result<(), TaggerError> {
    xtag::delete_tags(path)
}

fn list(path: &PathBuf) -> Result<(), TaggerError> {
    println!("{}", path.display());
    let container = xtag::get_tags(&path)?;
    for (tag, value) in container.iter().sorted() {
        match value {
            Some(value) => println!("  {}={}", tag, value),
            None => println!("  {}", tag),
        }
    }
    Ok(())
}

fn remove(path: &PathBuf, tags: &HashMap<String, Option<String>>) -> Result<(), TaggerError> {
    let mut container = xtag::get_tags(&path)?;

    for tag in tags.keys() {
        println!("Removing {} from {}", tag, path.display());
        container.remove(tag);
    }
    xtag::set_tags(&path, &container)?;
    Ok(())
}

fn find(path: &PathBuf, search: &xtag::Searcher) -> Result<(), TaggerError> {
    let tags = xtag::get_tags(&path)?;
    if search.is_match(&tags) {
        println!("{}", path.display());
    }
    Ok(())
}

fn rename(path: &PathBuf, find: &str, replace: &str) -> Result<(), TaggerError> {
    let tags = xtag::get_tags(&path)?;
    let tags = xtag::rename(find, replace, tags)?;
    xtag::set_tags(&path, &tags)?;
    Ok(())
}

fn tags(path: &PathBuf, all: &mut HashMap<String, HashSet<String>>) -> Result<(), TaggerError> {
    let tags = xtag::get_tags(&path)?;
    for (key, value) in tags {
        match value {
            Some(value) => {
                all.entry(key).or_insert(HashSet::new()).insert(value);
                ()
            }
            None => {
                all.entry(key).or_insert(HashSet::new());
                ()
            }
        }
    }
    Ok(())
}

fn do_for_all<A>(globs: &Vec<PathBuf>, mut func: A) -> Result<(), Box<dyn Error>>
where
    A: FnMut(&PathBuf) -> Result<(), TaggerError>,
{
    let options = MatchOptions {
        ..Default::default()
    };

    for glob in globs {
        let glob = glob.to_str().ok_or("Could not convert path to string")?;
        let globber = glob_with(glob, options)?;
        for entry in globber {
            func(&entry?)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Add { tags, globs } => {
            let map = xtag::csl_to_map(tags)?;
            do_for_all(globs, |path| add(path, &map))
        }
        Commands::Remove { tags, globs } => {
            let map = xtag::csl_to_map(tags)?;
            do_for_all(globs, |path| remove(path, &map))
        }
        Commands::Delete { globs } => do_for_all(globs, |path| delete(path)),
        Commands::List { globs } => do_for_all(globs, |path| list(path)),
        Commands::Find { term, globs } => {
            let search = xtag::compile_search(term)?;
            do_for_all(globs, |path| find(path, &search))
        }
        Commands::Rename {
            find,
            replace,
            globs,
        } => do_for_all(globs, |path| rename(path, find, replace)),
        Commands::Tags { globs } => {
            let mut all: HashMap<String, HashSet<String>> = HashMap::new();
            do_for_all(globs, |path| tags(path, &mut all))?;
            for key in all.keys() {
                println!("{key}");
            }
            Ok(())
        }
    }
}
