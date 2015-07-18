use super::trie::ParserTrie;
use super::result::MatchResult;
use super::pattern::{file, Pattern};
use super::pattern::source::PatternSource;

pub mod builder;

#[derive(Clone, Debug)]
pub struct Matcher {
    parser: ParserTrie
}

impl Matcher {
    pub fn from_file(pattern_file_path: &str) -> Result<Matcher, builder::BuildError> {
        let file = try!(file::PlainPatternFile::open(pattern_file_path));
        Matcher::drain_into(&mut file.into_iter())
    }

    pub fn from_json_file(pattern_file_path: &str) -> Result<Matcher, builder::BuildError> {
        let file = try!(file::SerializedPatternFile::open(pattern_file_path));
        Matcher::drain_into(&mut file.into_iter())
    }

    pub fn drain_into(source: &mut PatternSource) -> Result<Matcher, builder::BuildError> {
        let mut trie = ParserTrie::new();
        try!(builder::Builder::drain_into(source, &mut trie));
        Ok(Matcher{ parser: trie })
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
        self.parser.parse(text)
    }
}
