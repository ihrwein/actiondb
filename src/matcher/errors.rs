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
pub enum FromJsonError<'a, 'b> {
    File(file::Error),
    TestPairs(testmessage::TestPairsError<'a, 'b>),
    TestMessageDoesntMatch
}

impl<'a, 'b> From<file::Error> for FromJsonError<'a, 'b> {
    fn from(error: file::Error) -> FromJsonError<'a, 'b> {
        FromJsonError::File(error)
    }
}

impl<'a, 'b> From<testmessage::TestPairsError<'a, 'b>> for FromJsonError<'a, 'b> {
    fn from(error: testmessage::TestPairsError) -> FromJsonError<'a, 'b> {
        FromJsonError::TestPairs(error)
    }
}
