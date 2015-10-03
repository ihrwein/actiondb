use matcher::factory::MatcherFactory;
use matcher::trie::ParserTrie;

pub struct TrieMatcherFactory;

impl MatcherFactory for TrieMatcherFactory {
    type Matcher = ParserTrie;

    fn new_matcher() -> Self::Matcher {
        ParserTrie::new()
    }
}
