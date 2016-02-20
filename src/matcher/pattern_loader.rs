use super::pattern::file;
use matcher::BuildError;
use matcher::MatcherFactory;
use matcher::FromPatternSource;

use std::path;
use std::ffi;

pub struct PatternLoader;

impl PatternLoader {
    pub fn from_json_file<F>(pattern_file_path: &str) -> Result<F::Matcher, BuildError>
        where F: MatcherFactory
    {
        let file = try!(file::PatternFile::open(pattern_file_path));
        F::Matcher::from_source::<F>(&mut file.into_iter())
    }

    pub fn from_file<F>(pattern_file_path: &str) -> Result<F::Matcher, BuildError>
        where F: MatcherFactory
    {
        let path = path::Path::new(pattern_file_path);
        match path.extension() {
            Some(extension) => {
                PatternLoader::from_file_based_on_extension::<F>(extension, pattern_file_path)
            }
            None => Err(BuildError::UnsupportedFileExtension),
        }
    }

    fn from_file_based_on_extension<F>(extension: &ffi::OsStr,
                                       pattern_file_path: &str)
                                       -> Result<F::Matcher, BuildError>
        where F: MatcherFactory
    {
        match try!(extension.to_str().ok_or(BuildError::NotUtf8FileName)) {
            "json" => PatternLoader::from_json_file::<F>(pattern_file_path),
            _ => Err(BuildError::UnsupportedFileExtension),
        }
    }
}
