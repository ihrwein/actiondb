use matcher::pattern::Pattern;

pub struct PatternFile {
    pub patterns: Vec<Pattern>,
}

impl PatternFile {
    pub fn patterns(&self) -> &Vec<Pattern> {
        &self.patterns
    }
}
