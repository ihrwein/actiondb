use std::collections::BTreeMap;

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

    pub fn insert(&mut self, key: &'a str, value: &'b str) {
        self.values.insert(key, value);
    }

    pub fn pattern(&self) -> &Pattern {
        self.pattern
    }

    pub fn values(&self) -> &BTreeMap<&'a str, &'b str> {
        &self.values
    }
}
