use grammar::parser::ParseError;
use std::io;
use std::fmt;
use std::error;

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

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Error {
        Error::PatternParse(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Error::PatternParse(ref error) => error.fmt(formatter),
            &Error::IO(ref error) => error.fmt(formatter)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::PatternParse(ref error) => error.description(),
            &Error::IO(ref error) => error.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::PatternParse(ref error) => error.cause(),
            &Error::IO(ref error) => error.cause()
        }
    }
}
