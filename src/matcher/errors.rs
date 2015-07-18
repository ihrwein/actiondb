use matcher::pattern::file;
use super::pattern::testmessage;

#[derive(Debug)]
pub enum FromJsonError {
    File(file::serialized::Error),
    TestPairs(testmessage::TestPairsError),
    TestMessageDoesntMatch
}

impl From<file::serialized::Error> for FromJsonError {
    fn from(error: file::serialized::Error) -> FromJsonError {
        FromJsonError::File(error)
    }
}

impl From<testmessage::TestPairsError> for FromJsonError {
    fn from(error: testmessage::TestPairsError) -> FromJsonError {
        FromJsonError::TestPairs(error)
    }
}
