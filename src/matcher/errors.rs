use std::io::Error;
use grammar::parser::ParseError;

#[derive(Debug)]
pub enum BuildFromFileError {
    PatternParseError(ParseError),
    IOError(Error)
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
