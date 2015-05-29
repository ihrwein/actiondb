use std::cmp::{Ord, Ordering};
use std::borrow::Borrow;

use node::Node;

pub struct LiteralNode <'a, 'b, T: Borrow<str>> {
    literal: T,
    node: Option<Box<Node<'a, 'b>>>,
}

impl <'a, 'b, T: Borrow<str>> LiteralNode<'a, 'b, T> {
    pub fn new(literal: T) -> LiteralNode<'a, 'b, T> {
        LiteralNode{ literal: literal,
                     node: None}
    }
}

impl <'a, 'b, T: Borrow<str> + Eq> Eq for LiteralNode<'a, 'b, T> {}

impl <'a, 'b, T: Borrow<str> + PartialEq> PartialEq for LiteralNode<'a, 'b, T> {
    fn eq(&self, other: &Self) -> bool {
        self.literal == other.literal
    }

    fn ne(&self, other: &Self) -> bool {
        self.literal != other.literal
    }
}

impl <'a, 'b, T: Borrow<str> + Ord> Ord for LiteralNode<'a, 'b, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.literal.cmp(&other.literal)
    }
}

impl <'a, 'b, T: Borrow<str> + Ord> PartialOrd for LiteralNode<'a, 'b, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.literal.cmp(&other.literal))
    }
}
