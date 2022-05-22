mod args;

pub use crate::args::{custom_validation, Args};
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

fn print_file(path: &PathBuf, hyperlink: bool) {
    if hyperlink {
        println!(
            "\u{001b}]8;;file://{}\u{001b}\\{}\u{001b}]8;;\u{001b}\\",
            path.display(), // FIXME canonicalize here?
            path.display()
        )
    } else {
        println!("{}", path.display())
    }
}

pub fn list_file(path: &PathBuf, hyperlink: bool) {
    print_file(&path, hyperlink);
}

pub fn show_file(path: &PathBuf, tags: &HashMap<String, Option<String>>, hyperlink: bool) {
    print_file(&path, hyperlink);
    for (tag, value) in tags.iter().sorted() {
        match value {
            Some(value) => println!("  {tag}={value}"),
            None => println!("  {tag}"),
        }
    }
}

pub fn collect_tags(
    tags: &HashMap<String, Option<String>>,
    collection: &mut HashMap<String, HashSet<String>>,
) {
    for (key, value) in tags {
        match value {
            Some(value) => {
                collection
                    .entry(key.to_owned())
                    .or_insert(HashSet::new())
                    .insert(value.to_owned());
                ()
            }
            None => {
                collection.entry(key.to_owned()).or_insert(HashSet::new());
                ()
            }
        }
    }
}

fn handle_endpoint(
    path: &PathBuf,
    args: &Args,
    all_tags: &mut HashMap<String, HashSet<String>>,
) -> Result<()> {
    let mut tags = xtag::get_tags(&path)?;
    let mut tags_possibly_changed = false;

    if let Some(filter) = &args.filter {
        if !filter.is_match(&tags) {
            return Ok(());
        }
    }

    if let (Some(find), Some(replace)) = (&args.find, &args.replace) {
        tags = xtag::rename(&find, &replace, tags)?;
        tags_possibly_changed = true;
    }

    if let Some(remove_tags) = &args.remove {
        for tag in remove_tags.keys() {
            tags.remove(tag);
        }
        tags_possibly_changed = true;
    }

    if let Some(add_tags) = &args.add {
        tags.extend(add_tags.to_owned());
        tags_possibly_changed = true;
    }

    if args.list {
        list_file(&path, args.hyperlink);
    }

    if args.show {
        show_file(&path, &tags, args.hyperlink);
    }

    if args.tags {
        collect_tags(&tags, all_tags);
    }

    if tags_possibly_changed && !args.dry_run {
        xtag::set_tags(&path, &tags)?;
    }

    if args.delete {
        xtag::delete_tags(&path)?;
    }

    Ok(())
}

fn handle_path(
    path: &PathBuf,
    args: &Args,
    all_tags: &mut HashMap<String, HashSet<String>>,
) -> Result<()> {
    if path.is_dir() {
        // The directory is also handled
        handle_endpoint(path, args, all_tags)?;
        for entry in fs::read_dir(path)? {
            handle_path(&entry?.path(), args, all_tags)?;
        }
    } else {
        handle_endpoint(path, args, all_tags)?;
    }
    Ok(())
}

pub fn run(args: &Args) -> Result<()> {
    let mut all_tags: HashMap<String, HashSet<String>> = HashMap::new();

    for path in &args.paths {
        handle_path(path, args, &mut all_tags)?;
    }

    if args.tags {
        for key in all_tags.keys().sorted() {
            println!("{key}");
        }
    }

    Ok(())
}
