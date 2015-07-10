use uuid::Uuid;

use matcher::trie::node::{CompiledPattern, TokenType};

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
            pattern: Vec::new()
        }
    }

    pub fn with_random_uuid() -> Pattern {
        Pattern{
            uuid: Uuid::new_v4(),
            name: None,
            pattern: Vec::new()
        }
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
