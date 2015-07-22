use grammar::parser::ParseError;
use std::io;

#[derive(Debug)]
pub enum Error {
    PatternParse(ParseError),
    IO(io::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IO(error)
    }
}
