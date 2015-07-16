use std::io;
use grammar::parser::ParseError;
use super::pattern::file;
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
pub enum FromJsonError<'a> {
    File(file::Error),
    TestPairs(testmessage::TestPairsError<'a>),
    TestMessageDoesntMatch
}

impl<'a> From<file::Error> for FromJsonError<'a> {
    fn from(error: file::Error) -> FromJsonError<'a> {
        FromJsonError::File(error)
    }
}

impl<'a> From<testmessage::TestPairsError<'a>> for FromJsonError<'a> {
    fn from(error: testmessage::TestPairsError) -> FromJsonError {
        FromJsonError::TestPairs(error)
    }
}
