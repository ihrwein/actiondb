use matcher::pattern::Pattern;
use serde::json;

use std::fs;
use std::io::Read;

pub use self::error::{DeserError, Error};

mod deser;
mod error;

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
