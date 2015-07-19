use super::trie::ParserTrie;
use super::result::MatchResult;
use super::pattern::file;
use super::pattern::source::PatternSource;
use super::matcher::builder;

#[derive(Clone, Debug)]
pub struct Factory {
    parser: ParserTrie
}

impl Factory {
    pub fn from_plain_file(pattern_file_path: &str) -> Result<Factory, builder::BuildError> {
        let file = try!(file::PlainPatternFile::open(pattern_file_path));
        Factory::drain_into(&mut file.into_iter())
    }

    pub fn from_json_file(pattern_file_path: &str) -> Result<Factory, builder::BuildError> {
        let file = try!(file::SerializedPatternFile::open(pattern_file_path));
        Factory::drain_into(&mut file.into_iter())
    }

    pub fn drain_into(source: &mut PatternSource) -> Result<Factory, builder::BuildError> {
        let mut trie = ParserTrie::new();
        try!(builder::Builder::drain_into(source, &mut trie));
        Ok(Factory{ parser: trie })
    }
}
