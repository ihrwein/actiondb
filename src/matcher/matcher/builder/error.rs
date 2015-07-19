use matcher::errors::FromJsonError;
use matcher::pattern::file::serialized;
use matcher::pattern::file::plain;
use matcher::pattern::testmessage;

#[derive(Debug)]
pub enum BuildError {
    FromSerialized(FromJsonError),
    FromPlain(plain::Error),
    TestPairs(testmessage::TestPairsError),
    TestMessageDoesntMatch
}

impl From<FromJsonError> for BuildError {
    fn from(error: FromJsonError) -> BuildError {
        BuildError::FromSerialized(error)
    }
}

impl From<serialized::Error> for BuildError {
    fn from(error: serialized::Error) -> BuildError {
        BuildError::FromSerialized(FromJsonError::from(error))
    }
}

impl From<plain::Error> for BuildError {
    fn from(error: plain::Error) -> BuildError {
        BuildError::FromPlain(error)
    }
}
