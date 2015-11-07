use matcher::Matcher;
use super::SuffixTree;
use matcher::result::MatchResult;
use matcher::pattern::Pattern;
use matcher::trie::node::interface::SuffixTree as STree;

impl Matcher for SuffixTree {
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
