use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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

pub fn add_tags(path: &PathBuf, tags: &HashMap<String, Option<String>>) -> Result<()> {
    let mut container = xtag::get_tags(&path)?;
    container.extend(tags.clone());
    xtag::set_tags(&path, &container)?;
    Ok(())
}

pub fn remove_tags(path: &PathBuf, tags: &HashMap<String, Option<String>>) -> Result<()> {
    let mut container = xtag::get_tags(&path)?;

    for tag in tags.keys() {
        container.remove(tag);
    }
    xtag::set_tags(&path, &container)?;
    Ok(())
}

pub fn delete_tags(path: &PathBuf) -> Result<()> {
    xtag::delete_tags(&path)?;
    Ok(())
}

pub fn list_file(path: &PathBuf, hyperlink: bool) -> Result<()> {
    print_file(&path, hyperlink);
    Ok(())
}

pub fn show_file(path: &PathBuf, hyperlink: bool) -> Result<()> {
    print_file(&path, hyperlink);
    let container = xtag::get_tags(&path)?;
    for (tag, value) in container.iter().sorted() {
        match value {
            Some(value) => println!("  {tag}={value}"),
            None => println!("  {tag}"),
        }
    }
    Ok(())
}

pub fn collect_tags(
    path: &PathBuf,
    collection: &mut HashMap<String, HashSet<String>>,
) -> Result<()> {
    let tags = xtag::get_tags(&path)?;
    for (key, value) in tags {
        match value {
            Some(value) => {
                collection
                    .entry(key)
                    .or_insert(HashSet::new())
                    .insert(value);
                ()
            }
            None => {
                collection.entry(key).or_insert(HashSet::new());
                ()
            }
        }
    }
    Ok(())
}

pub fn rename_tags(path: &PathBuf, find: &str, replace: &str) -> Result<()> {
    let tags = xtag::get_tags(&path)?;
    let tags = xtag::rename(find, replace, tags)?;
    xtag::set_tags(&path, &tags)?;
    Ok(())
}
