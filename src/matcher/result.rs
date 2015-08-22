use std::collections::BTreeMap;

use parsers::ParseResult;
use matcher::Pattern;

#[derive(Debug)]
pub struct MatchResult<'a, 'b> {
    pattern: &'a Pattern,
    values: BTreeMap<&'a str, &'b str>
}

impl <'a, 'b> MatchResult<'a, 'b> {
    pub fn new(pattern: &'a Pattern) -> MatchResult<'a, 'b> {
        MatchResult{
            pattern: pattern,
            values: BTreeMap::new()
        }
    }

    pub fn insert(&mut self, result: ParseResult<'a, 'b>) {
        if let Some(name) = result.parser().name() {
            self.values.insert(name, result.value());
        }
    }

    pub fn pattern(&self) -> &Pattern {
        self.pattern
    }

    pub fn values(&self) -> &BTreeMap<&'a str, &'b str> {
        &self.values
    }
}
