use matcher::trie::parser_factory::TrieParserFactory;
use matcher::MatcherFactory;

use matcher::MatcherSuite;
use self::impls::SuffixTable;
use self::interface::SuffixArray;

mod interface;
mod impls;
#[cfg(test)]
mod test;

pub struct SuffixArrayMatcherFactory;

impl MatcherFactory for SuffixArrayMatcherFactory {
    type Matcher = SuffixTable;

    fn new_matcher() -> Self::Matcher {
        SuffixTable::new()
    }
}

#[derive(Clone)]
pub struct SuffixArrayMatcherSuite;

impl MatcherSuite for SuffixArrayMatcherSuite {
    type Matcher = SuffixTable;
    type ParserFactory = TrieParserFactory;
    type MatcherFactory = SuffixArrayMatcherFactory;
}
