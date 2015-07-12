use uuid::{self, Uuid};
use yaml_rust::yaml::Yaml;

use grammar::parser::ParseError;
use grammar::pattern_parser;
use matcher::trie::node::{CompiledPattern, TokenType};

use std::str::FromStr;
use std::borrow::Borrow;

#[derive(Clone, Debug)]
pub struct Pattern {
    name: Option<String>,
    uuid: Uuid,
    pattern: CompiledPattern
}

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
        println!("doc: {:?}", doc);
        let mut pattern = None;
        let mut name = None;
        let mut uuid = None;
        if let Some(hash) = doc.as_hash() {
            pattern = hash.get(&Yaml::String("pattern".to_string()));
            name = hash.get(&Yaml::String("name".to_string()));
            uuid = hash.get(&Yaml::String("uuid".to_string()));
            println!("name: {:?} uuid: {:?} pattern {:?}", name, uuid, pattern);
        }

        let name_final = {
            let unwrapped = try!(name.ok_or(FromYamlError::SchemaError));
            unwrapped.as_str().map(|x| x.to_string())
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

#[cfg(test)]
mod test {
    use super::Pattern;
    use yaml_rust::{Yaml, YamlLoader};
    use uuid::Uuid;

    #[test]
    fn test_given_yaml_document_when_it_does_not_contain_errors_then_pattern_can_be_created_from_it() {
        let buffer = r#"
name: SSH_DISCONNECT
uuid: 9a49c47d-29e9-4072-be84-3b76c6814743
pattern: "Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} lobotomy sshd[%{INT:pid}]: Received disconnect from %{GREEDY:ipaddr}: %{INT:dunno}: disconnected by user"
        "#;

        let docs = YamlLoader::load_from_str(buffer).unwrap();
        assert_eq!(docs.len(), 1);
        let doc = &docs[0];
        let expected_uuid = Uuid::parse_str("9a49c47d-29e9-4072-be84-3b76c6814743").ok().unwrap();
        let pattern = Pattern::from_yaml(doc).ok().unwrap();
        assert_eq!(pattern.name(), Some("SSH_DISCONNECT"));
        assert_eq!(pattern.uuid().as_bytes(), expected_uuid.as_bytes());
        assert_eq!(pattern.pattern().len(), 15);
    }
}
