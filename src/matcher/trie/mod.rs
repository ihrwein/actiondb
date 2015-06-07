pub mod node;
mod trie;

use self::node::{LiteralNode, ParserNode};
use parsers::Parser;
pub use self::trie::ParserTrie;

pub trait TrieOperations {
    fn insert_literal(&mut self, literal: &str) -> &mut LiteralNode;
    fn insert_parser(&mut self, parser: Box<Parser>) -> &mut ParserNode;
}
