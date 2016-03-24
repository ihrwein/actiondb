use super::pattern::file::PatternFile;
use matcher::BuildError;
use matcher::MatcherFactory;
use matcher::FromPatternSource;

use std::path;
use std::fs::File;
use std::io::Read;

use serde_json;
use serde_yaml;

pub struct PatternLoader;

impl PatternLoader {
    fn read(pattern_file_path: &str) -> Result<String, BuildError>
    {
        let mut buffer = String::new();
        let mut file = try!(File::open(pattern_file_path));
        let _ = file.read_to_string(&mut buffer);
        Ok(buffer)
    }

    pub fn from_file_ignore_errors<F>(pattern_file_path: &str) -> Result<F::Matcher, BuildError>
        where F: MatcherFactory
    {
        let file = try!(PatternLoader::load_file(pattern_file_path));
        Ok(F::Matcher::from_source_ignore_errors::<F>(&mut file.into_iter()))
    }

    pub fn from_file<F>(pattern_file_path: &str) -> Result<F::Matcher, BuildError>
        where F: MatcherFactory
    {
        let file = try!(PatternLoader::load_file(pattern_file_path));
        F::Matcher::from_source::<F>(&mut file.into_iter())
    }

    pub fn load_file(pattern_file_path: &str) -> Result<PatternFile, BuildError>
    {
        let path = path::Path::new(pattern_file_path);
        match path.extension() {
            Some(extension) => {
                match try!(extension.to_str().ok_or(BuildError::NotUtf8FileName)) {
                    "json" => {
                        let content = try!(PatternLoader::read(pattern_file_path));
                        let file = try!(serde_json::from_str::<PatternFile>(&content));
                        Ok(file)
                    },
                    "yaml" | "yml" | "YAML" | "YML" => {
                        let content = try!(PatternLoader::read(pattern_file_path));
                        let file = try!(serde_yaml::from_str::<PatternFile>(&content));
                        Ok(file)
                    },
                    _ => Err(BuildError::UnsupportedFileExtension),
                }
            }
            None => Err(BuildError::UnsupportedFileExtension),
        }
    }
}
