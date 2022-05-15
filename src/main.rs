mod args;
mod lib;

use anyhow::Result;
use args::Args;
use clap::Parser;
use glob::MatchOptions;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

// FIXME get rid of unwrap and propagate
fn build_iter<'a>(
    globs: &'a Vec<PathBuf>,
    filter: &'a Option<String>,
) -> Result<Box<dyn Iterator<Item = PathBuf> + 'a>> {
    let glob_options = MatchOptions {
        ..Default::default()
    };

    let search;
    let mut path_iter: Box<dyn Iterator<Item = PathBuf>> = Box::new(
        globs
            .iter()
            .flat_map(move |glob| {
                let glob = glob
                    .to_str()
                    .ok_or("Could not convert path to string")
                    .unwrap();
                glob::glob_with(glob, glob_options).unwrap()
            })
            .map(|path| path.unwrap()),
    );

    // Filter the glob files first
    if let Some(term) = filter {
        search = xtag::compile_search(&term)?;
        path_iter = Box::new(path_iter.filter(move |path| {
            let tags = xtag::get_tags(path).unwrap();
            search.is_match(&tags)
        }));
    };
    Ok(path_iter)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut all_tags: HashMap<String, HashSet<String>> = HashMap::new();

    for path in build_iter(&args.globs, &args.filter)? {
        // FIXME rename here

        if let Some(tags) = &args.remove {
            lib::remove_tags(&path, tags)?;
        }

        if let Some(tags) = &args.add {
            lib::add_tags(&path, tags)?;
        }

        if let (Some(find), Some(replace)) = (&args.find, &args.replace) {
            lib::rename_tags(&path, &find, &replace)?;
        }

        if args.delete {
            lib::delete_tags(&path)?;
        }

        if args.list {
            lib::list_file(&path, args.hyperlink)?;
        }

        if args.show {
            lib::show_file(&path, args.hyperlink)?;
        }

        if args.tags {
            lib::collect_tags(&path, &mut all_tags)?;
        }
    }

    if args.tags {
        for key in all_tags.keys() {
            println!("{key}");
        }
    }

    Ok(())
}
