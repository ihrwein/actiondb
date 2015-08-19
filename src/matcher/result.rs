use matcher::Pattern;

#[derive(Debug)]
pub struct MatchResult<'a, 'b> {
    key_value_pairs: Vec<(&'a str, &'b str)>,
    pattern: &'a Pattern
}

impl <'a, 'b> MatchResult<'a, 'b> {
    pub fn new(pattern: &'a Pattern) -> MatchResult<'a, 'b> {
        MatchResult{
            key_value_pairs: Vec::new(),
            pattern: pattern
        }
    }

    pub fn push_pair(&mut self, key: &'a str, value: &'b str) {
        self.key_value_pairs.push((key, value));
    }

    pub fn pattern(&self) -> &Pattern {
        self.pattern
    }

    pub fn values(&self) -> &Vec<(&'a str, &'b str)> {
        &self.key_value_pairs
    }
}
