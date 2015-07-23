use matcher::Matcher;
use super::ParserTrie;
use matcher::result::MatchResult;
use matcher::pattern::Pattern;

impl Matcher for ParserTrie {
    fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
        self.parse(text)
    }
    fn add_pattern(&mut self, pattern: Pattern) {
        self.insert(pattern);
    }
    fn boxed_clone(&self) -> Box<Matcher> {
        Box::new(self.clone())
    }
}
