use matcher::pattern::Pattern;
use serde_json;

use std::fs;
use std::io::Read;

use super::error::{DeserError, Error};

pub struct SerializedPatternFile {
    pub patterns: Vec<Pattern>,
}

impl SerializedPatternFile {
    pub fn open(path: &str) -> Result<SerializedPatternFile, Error> {
        let mut buffer = String::new();
        let mut file = try!(fs::File::open(path));

        try!(file.read_to_string(&mut buffer));
        serde_json::from_str::<SerializedPatternFile>(&buffer)
            .map_err(|error| Error::from(DeserError::from(error)))
    }

    pub fn patterns(&self) -> &Vec<Pattern> {
        &self.patterns
    }
}
