use matcher::errors::FromJsonError;
use matcher::pattern::file::serialized;
use matcher::pattern::file::plain;

#[derive(Debug)]
pub enum BuildError {
    FromJson(FromJsonError),
    FromPlain(plain::Error)
}

impl From<FromJsonError> for BuildError {
    fn from(error: FromJsonError) -> BuildError {
        BuildError::FromJson(error)
    }
}

impl From<serialized::Error> for BuildError {
    fn from(error: serialized::Error) -> BuildError {
        BuildError::FromJson(FromJsonError::from(error))
    }
}

impl From<plain::Error> for BuildError {
    fn from(error: plain::Error) -> BuildError {
        BuildError::FromPlain(error)
    }
}
