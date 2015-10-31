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
        let result = try!(SerializedPatternFile::deser(&buffer));
        Ok(result)
    }

    fn deser(buffer: &str) -> Result<SerializedPatternFile, DeserError> {
        let result = try!(serde_json::from_str::<SerializedPatternFile>(&buffer));
        Ok(result)
    }

    pub fn patterns(&self) -> &Vec<Pattern> {
        &self.patterns
    }
}
