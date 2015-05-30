use std::collections::BTreeMap;
use parsers::{Parser, SetParser};
use utils::SortedVec;
use matcher::trie::node::LiteralNode;
use matcher::trie::node::ParserNode;

pub type MatchResult<'a, 'b> = Option<BTreeMap<&'a str, &'b str>>;
pub type CompiledPattern<'a, 'b> = Vec<NodeType<'a, 'b>>;

pub enum NodeType<'a, 'b> {
    Parser(Box<Parser<'a, 'b>>),
    Literal(&'a str)
}

pub struct Node<'a, 'b> {
    literal_children: SortedVec<LiteralNode<'a, 'b>>,
    parser_children: Vec<ParserNode<'a, 'b>>
}

impl <'a, 'b, 'c> Node<'a, 'b> {
    pub fn new() -> Node<'a, 'b> {
        Node{ literal_children: SortedVec::new(),
              parser_children: Vec::new() }
    }

    pub fn parse(&mut self, value: &'b str) -> MatchResult<'c, 'b> {
        None
    }

    fn find(&mut self, literal: &str) -> (&mut Node<'a, 'b>, usize) {
        if self.literal_children.len() == 0 {
            return (self, 0);
        }
        (self, 0)
    }

    pub fn insert(&mut self, pattern: &CompiledPattern<'a, 'b>) -> Result<&str, &str> {
        let first = pattern.first();

        if let Some(value) = pattern.first() {
            match value {
                &NodeType::Literal(literal) => {
                    return self.insert_literal(literal);
                },
                &NodeType::Parser(ref parser) => {

                }
            }
        } else {
            // insert it here?
        }
        Err("err")
    }

    fn insert_literal(&mut self, literal: &'a str) -> Result<&str, &str> {
        let lit_node = &LiteralNode::new(literal);
        let pos = self.literal_children.find_pos(lit_node);
        Err("err")
    }
}
