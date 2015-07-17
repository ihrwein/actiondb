use matcher::pattern::Pattern;
use serde::json;
use serde;

use std::fs;
use std::io::{self, Read};

mod deser;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Deser(DeserError)
}

#[derive(Debug)]
pub enum DeserError {
    JSON(serde::json::Error)
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IO(error)
    }
}

impl From<json::Error> for DeserError {
    fn from(error: json::Error) -> DeserError {
        DeserError::JSON(error)
    }
}

impl From<DeserError> for Error {
    fn from(error: DeserError) -> Error {
        Error::Deser(error)
    }
}

pub struct File {
    pub patterns: Vec<Pattern>
}

impl File {
    pub fn open(path: &str) -> Result<File, Error> {
        let mut buffer = String::new();
        let mut file = try!(fs::File::open(path));

        try!(file.read_to_string(&mut buffer));
        json::from_str::<File>(&buffer).map_err(|error| Error::from(DeserError::from(error)))
    }

    pub fn patterns(&self) -> &Vec<Pattern> {
        &self.patterns
    }
}
