use matcher::suite::MatcherSuite;
use matcher::trie::TrieMatcher;
use matcher::trie::parser_factory::TrieParserFactory;
use matcher::trie::factory::TrieMatcherFactory;

pub struct TrieMatcherSuite;

impl MatcherSuite for TrieMatcherSuite {
    type Matcher = TrieMatcher;
    type ParserFactory = TrieParserFactory;
    type MatcherFactory = TrieMatcherFactory;
}
