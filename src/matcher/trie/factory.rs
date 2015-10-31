use matcher::factory::MatcherFactory;
use matcher::trie::TrieMatcher;

pub struct TrieMatcherFactory;

impl MatcherFactory for TrieMatcherFactory {
    type Matcher = TrieMatcher;

    fn new_matcher() -> Self::Matcher {
        TrieMatcher::new()
    }
}
