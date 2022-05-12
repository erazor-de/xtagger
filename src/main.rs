use clap::{Parser, Subcommand};
use glob::{glob_with, MatchOptions};
use itertools::Itertools;
use std::collections::HashMap;
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

    /// List all tags
    List {
        #[clap(parse(from_os_str), value_name = "GLOB")]
        globs: Vec<PathBuf>,
    },

    /// Find
    Find {
        /// Search term
        #[clap(value_name = "TERM")]
        term: String,

        #[clap(parse(from_os_str), value_name = "GLOB")]
        globs: Vec<PathBuf>,
    },

    Rename {
        #[clap(value_name = "TERM")]
        from: String,

        #[clap(value_name = "TERM")]
        to: String,

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

fn delete(path: &PathBuf, _: ()) -> Result<(), TaggerError> {
    xtag::delete_tags(path)
}

fn list(path: &PathBuf, _: ()) -> Result<(), TaggerError> {
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

fn rename(path: &PathBuf, terms: (&String, &String)) -> Result<(), TaggerError> {
    let tags = xtag::get_tags(&path)?;
    let tags = xtag::rename(terms.0, terms.1, tags)?;
    xtag::set_tags(&path, &tags)?;
    Ok(())
}

fn do_for_all<A: Copy>(
    globs: &Vec<PathBuf>,
    arg: A,
    func: fn(&PathBuf, A) -> Result<(), TaggerError>,
) -> Result<(), Box<dyn Error>> {
    let options = MatchOptions {
        ..Default::default()
    };

    for glob in globs {
        let glob = glob.to_str().ok_or("Could not convert path to string")?;
        let globber = glob_with(glob, options)?;
        for entry in globber {
            func(&entry?, arg)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Add { tags, globs } => do_for_all(globs, &xtag::csl_to_map(tags)?, add),
        Commands::Remove { tags, globs } => do_for_all(globs, &xtag::csl_to_map(tags)?, remove),
        Commands::Delete { globs } => do_for_all(globs, (), delete),
        Commands::List { globs } => do_for_all(globs, (), list),
        Commands::Find { term, globs } => do_for_all(globs, &xtag::compile_search(term)?, find),
        Commands::Rename { from, to, globs } => do_for_all(globs, (from, to), rename),
    }
}
