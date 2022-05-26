mod app;
mod args;

pub use crate::args::Args;
use anyhow::Result;
use app::App;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use xtag::XTags;

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

fn show_file(path: &PathBuf, tags: &XTags, hyperlink: bool) {
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

fn collect_tags(tags: &XTags, collection: &mut HashMap<String, HashSet<String>>) {
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

fn handle_endpoint<F>(path: &PathBuf, app: &App, output_callback: &mut F) -> Result<()>
where
    F: FnMut(&PathBuf, &XTags),
{
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
            if let Some(_) = tags.remove(tag) {
                tags_possibly_changed = true;
            }
        }
    }

    if let Some(add_tags) = &app.args.manipulate.add {
        tags.extend(add_tags.to_owned());
        tags_possibly_changed = true;
    }

    output_callback(&path, &tags);

    if tags_possibly_changed && !app.args.dry_run {
        xtag::set_tags(&path, &tags)?;
    }

    if app.args.manipulate.delete {
        xtag::delete_tags(&path)?;
    }

    Ok(())
}

fn handle_path<F>(path: &PathBuf, app: &App, output_callback: &mut F) -> Result<()>
where
    F: FnMut(&PathBuf, &XTags),
{
    if path.is_dir() {
        // The directory is also handled
        handle_endpoint(path, app, output_callback)?;
        for entry in fs::read_dir(path)? {
            handle_path(&entry?.path(), app, output_callback)?;
        }
    } else {
        handle_endpoint(path, app, output_callback)?;
    }
    Ok(())
}

fn handle_paths<F>(app: &App, output_callback: &mut F) -> Result<()>
where
    F: FnMut(&PathBuf, &XTags),
{
    for path in &app.args.paths {
        handle_path(path, &app, output_callback)?;
    }
    Ok(())
}

pub fn run() -> Result<()> {
    let app = App::new()?;

    if app.args.print.tags {
        let mut all_tags: HashMap<String, HashSet<String>> = HashMap::new();

        handle_paths(&app, &mut |_, tags| collect_tags(tags, &mut all_tags))?;

        print_tags(&all_tags);
    } else if app.args.print.list {
        handle_paths(&app, &mut |path, _| list_file(path, app.args.hyperlink))?;
    } else if app.args.print.show {
        handle_paths(&app, &mut |path, tags| {
            show_file(path, tags, app.args.hyperlink)
        })?;
    } else {
        handle_paths(&app, &mut |_, _| {})?;
    }

    Ok(())
}
