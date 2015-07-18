use std::iter;
use grammar::parser;
use std::io::{self, BufReader, BufRead};
use std::fs;
use matcher::pattern::Pattern;
use matcher::pattern::source::BuildResult;
use matcher::matcher::builder::BuildError;

use super::PlainPatternFile;

impl iter::IntoIterator for PlainPatternFile {
    type Item = BuildResult;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter{lines: self.reader().lines()}
    }
}

pub struct IntoIter {
    lines: io::Lines<io::BufReader<fs::File>>
}

impl Iterator for IntoIter {
    type Item = BuildResult;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(Ok(line)) => {
                match parser::pattern(&line) {
                    Ok(compiled_pattern) => {
                        let mut pattern = Pattern::with_random_uuid();
                        pattern.set_pattern(compiled_pattern);
                        Some(Ok(pattern))
                    },
                    Err(err) => {
                        Some(Err(BuildError::from(super::Error::PatternParse(err))))
                    }
                }
            },
            _ => {
                None
            }
        }
    }
}
