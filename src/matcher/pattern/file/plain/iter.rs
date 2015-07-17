use std::iter;
use grammar::parser;
use std::io::{self, BufReader, BufRead};
use std::fs;
use matcher::pattern::Pattern;

use super::PlainPatternFile;

impl iter::IntoIterator for PlainPatternFile {
    type Item = Pattern;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter{lines: self.reader().lines()}
    }
}

pub struct IntoIter {
    lines: io::Lines<io::BufReader<fs::File>>
}

impl Iterator for IntoIter {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(Ok(line)) => {
                match parser::pattern(&line) {
                    Ok(compiled_pattern) => {
                        let mut pattern = Pattern::with_random_uuid();
                        pattern.set_pattern(compiled_pattern);
                        Some(pattern)
                    },
                    Err(_) => {
                        None
                    }
                }
            },
            _ => {
                None
            }
        }
    }
}
