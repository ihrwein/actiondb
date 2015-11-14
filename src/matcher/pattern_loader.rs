use super::pattern::file;
use super::matcher::builder;
use super::matcher::Matcher;

use std::path;
use std::ffi;

pub struct PatternLoader;

impl PatternLoader {
    pub fn from_json_file<F>(pattern_file_path: &str) -> Result<F::Matcher, builder::BuildError>
        where F: MatcherFactory
    {
        let mut matcher = F::new_matcher();
        let file = try!(file::PatternFile::open(pattern_file_path));
        try!(builder::Builder::drain_into(&mut file.into_iter(), &mut matcher));
        Ok(matcher)
    }

    pub fn from_file<F>(pattern_file_path: &str) -> Result<F::Matcher, builder::BuildError>
        where F: MatcherFactory
    {
        let path = path::Path::new(pattern_file_path);
        match path.extension() {
            Some(extension) => {
                PatternLoader::from_file_based_on_extension::<F>(extension, pattern_file_path)
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
            "json" => PatternLoader::from_json_file::<F>(pattern_file_path),
            _ => Err(builder::BuildError::UnsupportedFileExtension),
        }
    }
}

pub trait MatcherFactory {
    type Matcher: Matcher;
    fn new_matcher() -> Self::Matcher;
}
