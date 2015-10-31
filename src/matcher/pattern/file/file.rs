use matcher::pattern::Pattern;
use serde_json;

use std::fs;
use std::io::Read;

use super::error::{DeserError, Error};

pub struct PatternFile {
    pub patterns: Vec<Pattern>,
}

impl PatternFile {
    pub fn open(path: &str) -> Result<PatternFile, Error> {
        let mut buffer = String::new();
        let mut file = try!(fs::File::open(path));

        try!(file.read_to_string(&mut buffer));
        let result = try!(PatternFile::deser(&buffer));
        Ok(result)
    }

    fn deser(buffer: &str) -> Result<PatternFile, DeserError> {
        let result = try!(serde_json::from_str::<PatternFile>(&buffer));
        Ok(result)
    }

    pub fn patterns(&self) -> &Vec<Pattern> {
        &self.patterns
    }
}
