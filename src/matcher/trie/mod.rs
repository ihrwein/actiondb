pub mod node;
mod trie;
mod matcher;

use self::node::{LiteralNode, ParserNode};
use parsers::Parser;
pub use self::trie::ParserTrie;
use matcher::Pattern;

pub trait TrieOperations {
    fn insert_literal(&mut self, literal: &str) -> &mut LiteralNode;
    fn insert_parser(&mut self, parser: Box<Parser>) -> &mut ParserNode;
}

pub trait HasPattern {
    fn set_pattern(&mut self, pattern: Pattern);
    fn pattern(&self) -> Option<&Pattern>;
}
