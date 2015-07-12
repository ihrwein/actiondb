use std::io::Error;
use grammar::parser::ParseError;

use yaml_rust::scanner::ScanError;

#[derive(Debug)]
pub enum BuildFromFileError {
    PatternParseError(ParseError),
    IOError(Error),
    YamlScanError(ScanError),
    FileFormatError
}

impl From<Error> for BuildFromFileError {
    fn from(error: Error) -> BuildFromFileError {
        BuildFromFileError::IOError(error)
    }
}

impl From<ParseError> for BuildFromFileError {
    fn from(error: ParseError) -> BuildFromFileError {
        BuildFromFileError::PatternParseError(error)
    }
}

impl From<ScanError> for BuildFromFileError {
    fn from(error: ScanError) -> BuildFromFileError {
        BuildFromFileError::YamlScanError(error)
    }
}
