use matcher::pattern::file::serialized;
use matcher::pattern::file::plain;
use matcher::pattern::testmessage;

#[derive(Debug)]
pub enum BuildError {
    FromSerialized(serialized::Error),
    FromPlain(plain::Error),
    TestPairs(testmessage::TestPairsError),
    UnsupportedFileExtension,
    NotUtf8FileName
}

impl From<serialized::Error> for BuildError {
    fn from(error: serialized::Error) -> BuildError {
        BuildError::FromSerialized(error)
    }
}

impl From<plain::Error> for BuildError {
    fn from(error: plain::Error) -> BuildError {
        BuildError::FromPlain(error)
    }
}

impl From<testmessage::TestPairsError> for BuildError {
    fn from(error: testmessage::TestPairsError) -> BuildError {
        BuildError::TestPairs(error)
    }
}
