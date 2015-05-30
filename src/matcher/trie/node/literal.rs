use std::cmp::{Ord, Ordering};
use std::borrow::Borrow;

use matcher::trie::node::Node;

pub struct LiteralNode <'a, 'b> {
    literal: &'a str,
    node: Option<Box<Node<'a, 'b>>>,
}

impl <'a, 'b> LiteralNode<'a, 'b> {
    pub fn new(literal: &'a str) -> LiteralNode<'a, 'b> {
        LiteralNode{ literal: literal,
                     node: None}
    }
}

impl <'a, 'b> Eq for LiteralNode<'a, 'b> {}

impl <'a, 'b> PartialEq for LiteralNode<'a, 'b> {
    fn eq(&self, other: &Self) -> bool {
        self.literal == other.literal
    }

    fn ne(&self, other: &Self) -> bool {
        self.literal != other.literal
    }
}

impl <'a, 'b> Ord for LiteralNode<'a, 'b> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.literal.cmp(&other.literal)
    }
}

impl <'a, 'b> PartialOrd for LiteralNode<'a, 'b> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.literal.cmp(&other.literal))
    }
}

impl <'a, 'b> Borrow<str> for LiteralNode<'a, 'b> {
    fn borrow(&self) -> &str {
        self.literal
    }
}
