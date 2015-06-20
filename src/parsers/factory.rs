use parsers::{IntParser, SetParser, Parser};

pub struct ParserFactory;

impl ParserFactory {
    pub fn from_type(parer_type: &str) -> Option<Box<Parser>> {
        match parer_type {
            "INT" => Some(Box::new(IntParser::new())),
            "SET" => Some(Box::new(SetParser::new())),
            _ => None
        }
    }
}
