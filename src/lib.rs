mod app;
mod args;
mod glob_iter;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use anyhow::Result;
use app::App;
use itertools::Itertools;
use xtag::XTags;

pub use crate::args::Arguments;
use crate::glob_iter::GlobIter;

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

// Works on files and directories, symbolic links don't have extended attributes
fn handle_endpoint<F>(
    path: &PathBuf,
    app: &App,
    copytags: &Option<XTags>,
    output_callback: &mut F,
) -> Result<()>
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

    if let Some(copytags) = copytags {
        tags.extend(copytags.to_owned());
        tags_possibly_changed = true;
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

    if !app.args.dry_run {
        if tags_possibly_changed {
            xtag::set_tags(&path, &tags)?;
        }

        if app.args.manipulate.delete {
            xtag::delete_tags(&path)?;
        }
    }

    Ok(())
}

fn handle_paths<F>(app: &App, output_callback: &mut F) -> Result<()>
where
    F: FnMut(&PathBuf, &XTags),
{
    let mut first = true;
    let mut copytags: Option<XTags> = None;

    for path in GlobIter::new(&app.args.globs)? {
        let path = path?;
        if first {
            copytags = Some(xtag::get_tags(&path)?);
            first = false;
        }
        handle_endpoint(&path, &app, &copytags, output_callback)?;
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
