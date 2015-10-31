use super::trie::ParserTrie;
use super::pattern::file;
use super::matcher::builder;
use super::matcher::Matcher;
use matcher::trie::factory::TrieMatcherFactory;

use std::path;
use std::ffi;

#[derive(Clone, Debug)]
pub struct Factory;

impl Factory {
    pub fn from_json_file(pattern_file_path: &str) -> Result<ParserTrie, builder::BuildError> {
        GenericFactory::from_json_file::<TrieMatcherFactory>(pattern_file_path)
    }

    pub fn from_file(pattern_file_path: &str) -> Result<ParserTrie, builder::BuildError> {
        GenericFactory::from_file::<TrieMatcherFactory>(pattern_file_path)
    }
}

pub struct GenericFactory;

impl GenericFactory {
    pub fn from_json_file<F>(pattern_file_path: &str) -> Result<F::Matcher, builder::BuildError>
        where F: MatcherFactory
    {
        let mut matcher = F::new_matcher();
        let file = try!(file::SerializedPatternFile::open(pattern_file_path));
        try!(builder::Builder::drain_into(&mut file.into_iter(), &mut matcher));
        Ok(matcher)
    }

    pub fn from_file<F>(pattern_file_path: &str) -> Result<F::Matcher, builder::BuildError>
        where F: MatcherFactory
    {
        let path = path::Path::new(pattern_file_path);
        match path.extension() {
            Some(extension) => {
                GenericFactory::from_file_based_on_extension::<F>(extension, pattern_file_path)
            }
            None => Err(builder::BuildError::UnsupportedFileExtension),
        }
    }

    fn from_file_based_on_extension<F>(extension: &ffi::OsStr,
                                       pattern_file_path: &str)
                                       -> Result<F::Matcher, builder::BuildError>
        where F: MatcherFactory
    {
        match try!(extension.to_str().ok_or(builder::BuildError::NotUtf8FileName)) {
            "json" => GenericFactory::from_json_file::<F>(pattern_file_path),
            _ => Err(builder::BuildError::UnsupportedFileExtension),
        }
    }
}

pub trait MatcherFactory {
    type Matcher: Matcher;
    fn new_matcher() -> Self::Matcher;
}
