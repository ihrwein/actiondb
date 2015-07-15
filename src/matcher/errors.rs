use std::io::Error;
use grammar::parser::ParseError;
use matcher::pattern::FromYamlError;

#[derive(Debug)]
pub enum BuildFromFileError {
    PatternParseError(ParseError),
    IOError(Error),
    FileFormatError,
    FromYamlError(FromYamlError)
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

impl From<FromYamlError> for BuildFromFileError {
    fn from(error: FromYamlError) -> BuildFromFileError {
        BuildFromFileError::FromYamlError(error)
    }
}
