use super::Pattern;
use serde::de::Deserialize;
use serde::json;
use serde;

use std::fs;
use std::io::{self, Read};

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

impl serde::Deserialize for File {
    fn deserialize<D>(deserializer: &mut D) -> Result<File, D::Error>
        where D: serde::de::Deserializer
    {
        deserializer.visit_named_map("File", FileVisitor)
    }
}

enum Field {
    PATTERNS,
}

impl serde::Deserialize for Field {
    fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
        where D: serde::de::Deserializer
    {
        struct FieldVisitor;

        impl serde::de::Visitor for FieldVisitor {
            type Value = Field;

            fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                where E: serde::de::Error
            {
                match value {
                    "patterns" => Ok(Field::PATTERNS),
                    _ => Err(serde::de::Error::syntax_error()),
                }
            }
        }

        deserializer.visit(FieldVisitor)
    }
}

struct FileVisitor;

impl serde::de::Visitor for FileVisitor {
    type Value = File;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<File, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut patterns: Option<Vec<Pattern>> = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Field::PATTERNS) => { patterns = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let patterns_final = match patterns {
            Some(patterns) => patterns,
            None => try!(visitor.missing_field("patterns")),
        };

        try!(visitor.end());

        Ok(File{ patterns: patterns_final })
    }
}
