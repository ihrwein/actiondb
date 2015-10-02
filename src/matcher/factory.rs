use super::trie::ParserTrie;
use super::pattern::file;
use super::matcher::builder;
use super::matcher::Matcher;

use std::path;
use std::ffi;

#[derive(Clone, Debug)]
pub struct Factory;

impl Factory {
    pub fn from_json_file(pattern_file_path: &str) -> Result<Box<Matcher>, builder::BuildError> {
        let mut matcher = Factory::new();
        let file = try!(file::SerializedPatternFile::open(pattern_file_path));
        try!(builder::Builder::drain_into(&mut file.into_iter(), &mut *matcher));
        Ok(matcher)
    }

    pub fn from_file(pattern_file_path: &str) -> Result<Box<Matcher>, builder::BuildError> {
        let path = path::Path::new(pattern_file_path);
        match path.extension() {
            Some(extension) => {
                Factory::from_file_based_on_extension(extension, pattern_file_path)
            },
            None => Err(builder::BuildError::UnsupportedFileExtension)
        }
    }

    fn from_file_based_on_extension(extension: &ffi::OsStr, pattern_file_path: &str) -> Result<Box<Matcher>, builder::BuildError> {
        match try!(extension.to_str().ok_or(builder::BuildError::NotUtf8FileName)) {
            "json" => Factory::from_json_file(pattern_file_path),
            _ => Err(builder::BuildError::UnsupportedFileExtension)
        }
    }

    pub fn new() -> Box<Matcher> {
        let trie = ParserTrie::new();
        Box::new(trie)
    }
}
