use uuid::Uuid;
use serde_json;

use matcher::compiled_pattern::{CompiledPattern, TokenType};
use super::testmessage::TestMessage;

use std::borrow::Borrow;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Pattern {
    name: Option<String>,
    uuid: Uuid,
    pattern: CompiledPattern,
    values: Option<BTreeMap<String, String>>,
    tags: Option<Vec<String>>,
    test_messages: Option<Vec<TestMessage>>,
}

impl Pattern {
    pub fn with_uuid(uuid: Uuid) -> Pattern {
        Pattern::new(None, uuid, Vec::new(), None, None, None)
    }

    pub fn new(name: Option<String>,
               uuid: Uuid,
               pattern: CompiledPattern,
               test_messages: Option<Vec<TestMessage>>,
               values: Option<BTreeMap<String, String>>,
               tags: Option<Vec<String>>)
               -> Pattern {
        Pattern {
            uuid: uuid,
            name: name,
            pattern: pattern,
            values: values,
            tags: tags,
            test_messages: test_messages,
        }
    }

    pub fn with_random_uuid() -> Pattern {
        Pattern::with_uuid(Uuid::new_v4())
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

    pub fn values(&self) -> Option<&BTreeMap<String, String>> {
        self.values.as_ref()
    }

    pub fn tags(&self) -> Option<&[String]> {
        self.tags.as_ref().map(|tags| tags.borrow())
    }

    pub fn from_json(doc: &str) -> Result<Pattern, serde_json::error::Error> {
        serde_json::from_str::<Pattern>(doc)
    }

    pub fn set_pattern(&mut self, pattern: CompiledPattern) {
        self.pattern = pattern;
    }

    pub fn pop_first_token(&mut self) -> Option<TokenType> {
        if self.pattern.is_empty() {
            None
        } else {
            Some(self.pattern.remove(0))
        }
    }

    pub fn pop_test_message(&mut self) -> Option<TestMessage> {
        self.test_messages.as_mut().map_or(None, |x| x.pop())
    }
}
