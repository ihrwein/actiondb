use matcher::errors::FromJsonError;
use matcher::pattern::file::serialized;

#[derive(Debug)]
pub enum BuildError {
    FromJson(FromJsonError)
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
