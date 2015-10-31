use super::PatternFile;
use matcher::pattern::Pattern;
use matcher::pattern::source::BuildResult;

use std::iter;

impl iter::IntoIterator for PatternFile {
    type Item = BuildResult;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { patterns: self.patterns }
    }
}

pub struct IntoIter {
    patterns: Vec<Pattern>,
}

impl Iterator for IntoIter {
    type Item = BuildResult;

    fn next(&mut self) -> Option<Self::Item> {
        match self.patterns.pop() {
            Some(pattern) => Some(Ok(pattern)),
            None => None,
        }
    }
}
