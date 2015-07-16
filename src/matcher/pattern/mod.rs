use uuid::Uuid;
use serde::json;
use serde;

use matcher::trie::node::{CompiledPattern, TokenType};

use std::borrow::Borrow;

#[cfg(test)]
mod test;
mod deser;
pub mod file;

#[derive(Clone, Debug)]
pub struct Pattern {
    name: Option<String>,
    uuid: Uuid,
    pattern: CompiledPattern
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

    pub fn from_json(doc: &str) -> Result<Pattern, serde::json::error::Error> {
        json::from_str::<Pattern>(doc)
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
