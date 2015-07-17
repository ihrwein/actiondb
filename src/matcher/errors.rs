use std::io;
use grammar::parser::ParseError;
use matcher::pattern::file;
use super::pattern::testmessage;

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

#[derive(Debug)]
pub enum FromJsonError {
    File(file::Error),
    TestPairs(testmessage::TestPairsError),
    TestMessageDoesntMatch
}

impl From<file::Error> for FromJsonError {
    fn from(error: file::Error) -> FromJsonError {
        FromJsonError::File(error)
    }
}

impl From<testmessage::TestPairsError> for FromJsonError {
    fn from(error: testmessage::TestPairsError) -> FromJsonError {
        FromJsonError::TestPairs(error)
    }
}
