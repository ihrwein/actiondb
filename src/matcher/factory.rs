use super::trie::ParserTrie;
use super::pattern::file;
use super::pattern::source::PatternSource;
use super::matcher::builder;
use super::matcher::Matcher;

#[derive(Clone, Debug)]
pub struct Factory;

impl Factory {
    pub fn from_plain_file(pattern_file_path: &str) -> Result<Box<Matcher>, builder::BuildError> {
        let file = try!(file::PlainPatternFile::open(pattern_file_path));
        Factory::drain_into(&mut file.into_iter())
    }

    pub fn from_json_file(pattern_file_path: &str) -> Result<Box<Matcher>, builder::BuildError> {
        let file = try!(file::SerializedPatternFile::open(pattern_file_path));
        Factory::drain_into(&mut file.into_iter())
    }

    pub fn drain_into(source: &mut PatternSource) -> Result<Box<Matcher>, builder::BuildError> {
        let mut trie = ParserTrie::new();
        try!(builder::Builder::drain_into(source, &mut trie));
        Ok(Box::new(trie))
    }
}
