use std::io;
use grammar::parser::ParseError;

#[derive(Debug)]
pub enum BuildFromFileError {
    PatternParse(ParseError),
    IO(io::Error),
}

impl From<io::Error> for BuildFromFileError {
    fn from(error: io::Error) -> BuildFromFileError {
        BuildFromFileError::IO(error)
    }
}

impl From<ParseError> for BuildFromFileError {
    fn from(error: ParseError) -> BuildFromFileError {
        BuildFromFileError::PatternParse(error)
    }
}
