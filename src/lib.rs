mod app;
mod args;

pub use crate::args::Args;
use anyhow::Result;
use app::App;
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

fn list_file(path: &PathBuf, hyperlink: bool) {
    print_file(&path, hyperlink);
}

fn show_file(path: &PathBuf, tags: &HashMap<String, Option<String>>, hyperlink: bool) {
    print_file(&path, hyperlink);
    for (tag, value) in tags.iter().sorted() {
        match value {
            Some(value) => println!("  {tag}={value}"),
            None => println!("  {tag}"),
        }
    }
}

fn print_tags(tags: &HashMap<String, HashSet<String>>) {
    for key in tags.keys().sorted() {
        println!("{key}");
    }
}

fn collect_tags(
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
    app: &App,
    all_tags: &mut HashMap<String, HashSet<String>>,
) -> Result<()> {
    let mut tags = xtag::get_tags(&path)?;
    let mut tags_possibly_changed = false;

    if let Some(filter) = &app.filter {
        if !filter.is_match(&tags) {
            return Ok(());
        }
    }

    if let (Some(find), Some(replace)) = (
        &app.args.manipulate.rename.find,
        &app.args.manipulate.rename.replace,
    ) {
        tags = xtag::rename(&find, &replace, tags)?;
        tags_possibly_changed = true;
    }

    if let Some(remove_tags) = &app.args.manipulate.remove {
        for tag in remove_tags.keys() {
            tags.remove(tag);
        }
        tags_possibly_changed = true;
    }

    if let Some(add_tags) = &app.args.manipulate.add {
        tags.extend(add_tags.to_owned());
        tags_possibly_changed = true;
    }

    if app.args.print.list {
        list_file(&path, app.args.hyperlink);
    }

    if app.args.print.show {
        show_file(&path, &tags, app.args.hyperlink);
    }

    if app.args.print.tags {
        collect_tags(&tags, all_tags);
    }

    if tags_possibly_changed && !app.args.dry_run {
        xtag::set_tags(&path, &tags)?;
    }

    if app.args.manipulate.delete {
        xtag::delete_tags(&path)?;
    }

    Ok(())
}

fn handle_path(
    path: &PathBuf,
    app: &App,
    all_tags: &mut HashMap<String, HashSet<String>>,
) -> Result<()> {
    if path.is_dir() {
        // The directory is also handled
        handle_endpoint(path, app, all_tags)?;
        for entry in fs::read_dir(path)? {
            handle_path(&entry?.path(), app, all_tags)?;
        }
    } else {
        handle_endpoint(path, app, all_tags)?;
    }
    Ok(())
}

pub fn run() -> Result<()> {
    let app = App::new()?;
    let mut all_tags: HashMap<String, HashSet<String>> = HashMap::new();

    for path in &app.args.paths {
        handle_path(path, &app, &mut all_tags)?;
    }

    if app.args.print.tags {
        print_tags(&all_tags);
    }

    Ok(())
}
