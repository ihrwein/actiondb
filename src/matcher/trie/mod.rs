pub mod node;
pub mod parser_factory;
pub mod factory;
pub mod suite;
mod trie;
mod matcher;

use self::node::{LiteralNode, ParserNode};
use parsers::Parser;
pub use self::trie::TrieMatcher;
pub use self::suite::TrieMatcherSuite;
use matcher::Pattern;

pub trait TrieElement {
    fn insert_literal(&mut self, literal: &str) -> &mut LiteralNode;
    fn insert_parser(&mut self, parser: Box<Parser>) -> &mut ParserNode;
    fn set_pattern(&mut self, pattern: Pattern);
    fn pattern(&self) -> Option<&Pattern>;
}
