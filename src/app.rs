use anyhow::{anyhow, Result};
use clap::Parser;
use regex::Regex;
use xtag::Searcher;

use crate::args::Arguments;
use crate::glob_iter::GlobIter;

pub struct App {
    pub args: Arguments,
    pub filter: Option<Searcher>,
}

impl App {
    pub fn new() -> Result<Self> {
        let args = Arguments::parse();
        custom_validation(&args)?;
        let filter = get_searcher(&args)?;
        Ok(App { args, filter })
    }
}

fn get_searcher(args: &Arguments) -> Result<Option<Searcher>> {
    let filter = if let Some(term) = &args.filter.filter {
        Some(xtag::compile_search(term)?)
    } else if let Some(link) = &args.filter.bookmark {
        Some(xtag::get_bookmark(link)?)
    } else {
        None
    };
    Ok(filter)
}

// If find expression has capture group, replace expression needs $
// maybe escaping has been forgotten
fn check_capture_replace_group(find: &str, replace: &str) -> Result<()> {
    // Matches capture groups and named capture groups but not non-capture group
    // Doesn't match escaped parentheses
    let find_capture_group_start = Regex::new(r"(^|[^\\])\(([^\?]|\?P<)").unwrap();
    if find_capture_group_start.find(&find).is_some() {
        let find_dollar = Regex::new(r"(^|[^\\])\$[0-9{]").unwrap();
        if !find_dollar.find(&replace).is_some() {
            return Err(anyhow!(
                "find term contains capture group, but replace term no $"
            ));
        }
    }
    Ok(())
}

fn check_glob_counts(args: &Arguments) -> Result<()> {
    let mut iter = GlobIter::new(&args.globs)?;

    // Lazy counting only 2
    let mut count: u8 = 0;
    if let Some(_) = iter.next() {
        count += 1;
    }
    if let Some(_) = iter.next() {
        count += 1;
    }

    if args.manipulate.copy == true && count < 2 {
        return Err(anyhow!("copy mode needs at least 2 files"));
    }

    if (args.help == Some(false) || args.help == None) && count < 1 {
        return Err(anyhow!("no file to work on"));
    }

    Ok(())
}

fn custom_validation(args: &Arguments) -> Result<()> {
    if let (Some(find), Some(replace)) = (
        &args.manipulate.rename.find,
        &args.manipulate.rename.replace,
    ) {
        check_capture_replace_group(find, replace)?;
    }

    check_glob_counts(args)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_group_needs_replace_term() {
        // Capture group and replace term is ok
        assert!(check_capture_replace_group(r"a(b)c", r"$1").is_ok());
        assert!(check_capture_replace_group(r"a(?P<x>b)c", r"a${x}a").is_ok());
        assert!(check_capture_replace_group(r"(a)", r"$1").is_ok());

        // Capture group and no replace term is not ok
        assert!(check_capture_replace_group(r"(a)", r"a\$bc").is_err());
        assert!(check_capture_replace_group(r"(a)", r"a$bc").is_err());
        assert!(check_capture_replace_group(r"(a)", r"abc").is_err());

        // No capture group, everything is allowed
        assert!(check_capture_replace_group(r"\(a\)", r"$").is_ok());
        assert!(check_capture_replace_group(r"\(a\)", r"$1").is_ok());
    }
}
