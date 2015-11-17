use matcher::MatcherFactory;
use matcher::trie::SuffixTree;

pub struct TrieMatcherFactory;

impl MatcherFactory for TrieMatcherFactory {
    type Matcher = SuffixTree;

    fn new_matcher() -> Self::Matcher {
        SuffixTree::new()
    }
}
