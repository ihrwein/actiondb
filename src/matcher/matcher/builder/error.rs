use matcher::pattern::file;
use matcher::pattern::testmessage;

use std::fmt;
use std::error;

#[derive(Debug)]
pub enum BuildError {
    FromSerialized(file::Error),
    TestMessage(testmessage::Error),
    UnsupportedFileExtension,
    NotUtf8FileName,
}

impl From<file::Error> for BuildError {
    fn from(error: file::Error) -> BuildError {
        BuildError::FromSerialized(error)
    }
}

impl From<testmessage::Error> for BuildError {
    fn from(error: testmessage::Error) -> BuildError {
        BuildError::TestMessage(error)
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            BuildError::FromSerialized(ref error) => error.fmt(formatter),
            BuildError::TestMessage(ref error) => error.fmt(formatter),
            BuildError::UnsupportedFileExtension =>
                formatter.write_fmt(format_args!("The given file extension is not suppoted")),
            BuildError::NotUtf8FileName =>
                formatter.write_str("The given filename contains non Utf-8 characters"),
        }
    }
}

impl error::Error for BuildError {
    fn description(&self) -> &str {
        match *self {
            BuildError::FromSerialized(ref error) => error.description(),
            BuildError::TestMessage(ref error) => error.description(),
            BuildError::UnsupportedFileExtension => "The given file extension is not supported",
            BuildError::NotUtf8FileName => "The given filename contains non Utf-8 characters",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            BuildError::FromSerialized(ref error) => error.cause(),
            BuildError::TestMessage(ref error) => error.cause(),
            BuildError::UnsupportedFileExtension => None,
            BuildError::NotUtf8FileName => None,
        }
    }
}
