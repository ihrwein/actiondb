use std::iter;
use matcher::pattern::Pattern;
use matcher::pattern::source::BuildResult;
use matcher::trie::node::CompiledPattern;

use super::PlainPatternFile;

impl iter::IntoIterator for PlainPatternFile {
    type Item = BuildResult;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.patterns())
    }
}

pub struct IntoIter {
    patterns: Vec<CompiledPattern>
}

impl IntoIter {
    fn new(patterns: Vec<CompiledPattern>) -> IntoIter {
        IntoIter{
            patterns: patterns,
        }
    }
}

impl Iterator for IntoIter {
    type Item = BuildResult;

    fn next(&mut self) -> Option<Self::Item> {
        match self.patterns.pop() {
            Some(compiled_pattern) => {
                let mut pattern = Pattern::with_random_uuid();
                pattern.set_pattern(compiled_pattern);
                Some(Ok(pattern))
            },
            None => {
                None
            }
        }
    }
}
