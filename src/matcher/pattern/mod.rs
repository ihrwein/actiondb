use uuid::{self, Uuid};
use yaml_rust::yaml::Yaml;

use grammar::parser::ParseError;
use grammar::pattern_parser;
use matcher::trie::node::{CompiledPattern, TokenType};

use std::str::FromStr;
use std::borrow::Borrow;

#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
pub struct Pattern {
    name: Option<String>,
    uuid: Uuid,
    pattern: CompiledPattern
}

#[derive(Debug)]
pub enum FromYamlError {
    SchemaError,
    PatternParseError(ParseError),
    UuidError(uuid::ParseError)
}

impl From<uuid::ParseError> for FromYamlError {
    fn from(error: uuid::ParseError) -> FromYamlError {
        FromYamlError::UuidError(error)
    }
}

impl From<pattern_parser::ParseError> for FromYamlError {
    fn from(error: pattern_parser::ParseError) -> FromYamlError {
        FromYamlError::PatternParseError(error)
    }
}

impl Pattern {
    pub fn new(uuid: Uuid) -> Pattern {
        Pattern{
            uuid: uuid,
            name: None,
            pattern: Vec::new(),
        }
    }

    pub fn with_random_uuid() -> Pattern {
        Pattern::new(Uuid::new_v4())
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| x.borrow())
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn pattern(&self) -> &CompiledPattern {
        &self.pattern
    }

    pub fn from_yaml(doc: &Yaml) -> Result<Pattern, FromYamlError> {
        let mut pattern = None;
        let mut name = None;
        let mut uuid = None;
        if let Some(hash) = doc.as_hash() {
            pattern = hash.get(&Yaml::String("pattern".to_string()));
            name = hash.get(&Yaml::String("name".to_string()));
            uuid = hash.get(&Yaml::String("uuid".to_string()));
        }

        let name_final = {
            name.and_then(|x| x.as_str()).map(|x| x.to_string())
        };

        let uuid_final = {
            let unwrapped = try!(uuid.ok_or(FromYamlError::SchemaError));
            let raw = try!(unwrapped.as_str().ok_or(FromYamlError::SchemaError));
            try!(Uuid::from_str(raw))
        };

        let pattern_final = {
            let unwrapped = try!(pattern.ok_or(FromYamlError::SchemaError));
            let raw = try!(unwrapped.as_str().ok_or(FromYamlError::SchemaError));
            try!(pattern_parser::pattern(raw))
        };

        Ok(Pattern{name: name_final, uuid: uuid_final, pattern: pattern_final})
    }

    pub fn set_pattern(&mut self, pattern: CompiledPattern) {
        self.pattern = pattern;
    }

    pub fn pop_first_token(&mut self) -> TokenType {
        self.pattern.remove(0)
    }

    pub fn has_more_tokens(&self) -> bool {
        self.pattern.is_empty()
    }
}
