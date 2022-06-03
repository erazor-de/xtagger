use anyhow::{anyhow, Result};
use clap::Parser;
use regex::Regex;
use xtag::Searcher;

use crate::args::Args;

pub struct App {
    pub args: Args,
    pub filter: Option<Searcher>,
}

impl App {
    pub fn new() -> Result<Self> {
        let args = Args::parse();
        custom_validation(&args)?;
        let filter = get_searcher(&args)?;
        Ok(App { args, filter })
    }
}

fn get_searcher(args: &Args) -> Result<Option<Searcher>> {
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

fn custom_validation(args: &Args) -> Result<()> {
    if let (Some(find), Some(replace)) = (
        &args.manipulate.rename.find,
        &args.manipulate.rename.replace,
    ) {
        check_capture_replace_group(find, replace)?;
    }
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
