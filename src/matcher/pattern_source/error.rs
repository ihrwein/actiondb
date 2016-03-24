use matcher::pattern::testmessage;

use serde_json;
use serde_yaml;
use std::fmt;
use std::error;
use std::io;

#[derive(Debug)]
pub enum BuildError {
    TestMessage(testmessage::Error),
    Io(io::Error),
    DeserJson(serde_json::Error),
    DeserYaml(serde_yaml::Error),
    UnsupportedFileExtension,
    NotUtf8FileName,
}

impl From<testmessage::Error> for BuildError {
    fn from(error: testmessage::Error) -> BuildError {
        BuildError::TestMessage(error)
    }
}

impl From<io::Error> for BuildError {
    fn from(error: io::Error) -> BuildError {
        BuildError::Io(error)
    }
}

impl From<serde_json::Error> for BuildError {
    fn from(error: serde_json::Error) -> BuildError {
        BuildError::DeserJson(error)
    }
}

impl From<serde_yaml::Error> for BuildError {
    fn from(error: serde_yaml::Error) -> BuildError {
        BuildError::DeserYaml(error)
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            BuildError::TestMessage(ref error) => error.fmt(formatter),
            BuildError::Io(ref error) => error.fmt(formatter),
            BuildError::DeserJson(ref error) => error.fmt(formatter),
            BuildError::DeserYaml(ref error) => error.fmt(formatter),
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
            BuildError::TestMessage(ref error) => error.description(),
            BuildError::Io(ref error) => error.description(),
            BuildError::DeserJson(ref error) => error.description(),
            BuildError::DeserYaml(ref error) => error.description(),
            BuildError::UnsupportedFileExtension => "The given file extension is not supported",
            BuildError::NotUtf8FileName => "The given filename contains non Utf-8 characters",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            BuildError::TestMessage(ref error) => error.cause(),
            BuildError::Io(ref error) => error.cause(),
            BuildError::DeserJson(ref error) => error.cause(),
            BuildError::DeserYaml(ref error) => error.cause(),
            BuildError::UnsupportedFileExtension | BuildError::NotUtf8FileName => None,
        }
    }
}
