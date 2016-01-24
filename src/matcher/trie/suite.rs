use matcher::suite::MatcherSuite;
use matcher::trie::SuffixTree;
use matcher::trie::parser_factory::TrieParserFactory;
use matcher::trie::factory::TrieMatcherFactory;

#[derive(Clone)]
pub struct TrieMatcherSuite;

impl MatcherSuite for TrieMatcherSuite {
    type Matcher = SuffixTree;
    type ParserFactory = TrieParserFactory;
    type MatcherFactory = TrieMatcherFactory;
}
