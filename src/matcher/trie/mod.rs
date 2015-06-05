pub mod node;
mod trie;

use self::node::{LiteralNode, ParserNode};
use parsers::Parser;
pub use self::trie::PatternTrie;

pub trait TrieOperations<'a> {
    fn insert_literal(&mut self, literal: &str) -> &mut LiteralNode<'a>;
    fn insert_parser(&mut self, parser: Box<Parser<'a>>) -> &mut ParserNode<'a>;
}
