use std::io;
use std::fmt;
use std::error;

pub use self::deser::DeserError;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Deser(deser::DeserError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IO(error)
    }
}

impl From<DeserError> for Error {
    fn from(error: DeserError) -> Error {
        Error::Deser(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Error::IO(ref error) => error.fmt(formatter),
            &Error::Deser(ref error) => error.fmt(formatter),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::IO(ref error) => error.description(),
            &Error::Deser(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::IO(ref error) => error.cause(),
            &Error::Deser(ref error) => error.cause(),
        }
    }
}

mod deser {
    use std::fmt;
    use std::error;
    use serde_json;

    #[derive(Debug)]
    pub enum DeserError {
        JSON(serde_json::Error),
    }

    impl From<serde_json::Error> for DeserError {
        fn from(error: serde_json::Error) -> DeserError {
            DeserError::JSON(error)
        }
    }

    impl fmt::Display for DeserError {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            match self {
                &DeserError::JSON(ref error) => error.fmt(formatter),
            }
        }
    }

    impl error::Error for DeserError {
        fn description(&self) -> &str {
            match self {
                &DeserError::JSON(ref error) => error.description(),
            }
        }

        fn cause(&self) -> Option<&error::Error> {
            match self {
                &DeserError::JSON(ref error) => error.cause(),
            }
        }
    }
}
