use std::io;

pub use self::deser::DeserError;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Deser(deser::DeserError)
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

mod deser {
    use std::fmt;
    use std::error;
    use serde::json;

    #[derive(Debug)]
    pub enum DeserError {
        JSON(json::Error)
    }

    impl From<json::Error> for DeserError {
        fn from(error: json::Error) -> DeserError {
            DeserError::JSON(error)
        }
    }

    impl fmt::Display for DeserError {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            match self {
                &DeserError::JSON(ref error) => error.fmt(formatter)
            }
        }
    }

    impl error::Error for DeserError {
        fn description(&self) -> &str {
            match self {
                &DeserError::JSON(ref error) => error.description()
            }
        }

        fn cause(&self) -> Option<&error::Error> {
            match self {
                &DeserError::JSON(ref error) => error.cause()
            }
        }
    }
}
