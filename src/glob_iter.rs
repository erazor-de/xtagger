use std::path::PathBuf;

use anyhow::{anyhow, Result};
use glob::GlobError;

pub struct GlobIter {
    iter: Option<Box<dyn Iterator<Item = Result<PathBuf, GlobError>>>>,
}

impl GlobIter {
    // Builds chained iterator of all Globs keeping order
    pub fn new(globs: &Vec<PathBuf>) -> Result<Self> {
        let mut iter: Option<Box<dyn Iterator<Item = Result<PathBuf, GlobError>>>> = None;

        for glob in globs {
            let glob = glob
                .to_str()
                .ok_or(anyhow!("Could not convert path to string"))?;
            iter = match iter {
                Some(inner) => Some(Box::new(inner.chain(glob::glob(glob)?))),
                None => Some(Box::new(glob::glob(glob)?)),
            }
        }

        Ok(Self { iter })
    }
}

impl Iterator for GlobIter {
    type Item = Result<PathBuf, GlobError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            Some(inner) => inner.next(),
            None => None,
        }
    }
}
