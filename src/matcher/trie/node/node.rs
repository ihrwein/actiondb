use std::collections::BTreeMap;
use parsers::{Parser, SetParser};
use utils::{SortedVec, CommonPrefix};
use matcher::trie::node::LiteralNode;
use matcher::trie::node::ParserNode;
use matcher::trie::node::literal;

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

    pub fn parse(&mut self, value: &'b str) -> MatchResult<'a, 'b> {
        None
    }

    pub fn add_literal_node(&mut self, lnode: LiteralNode<'a, 'b>) {
        self.literal_children.push(lnode);
    }

    fn find(&mut self, literal: &str) -> (&mut Node<'a, 'b>, usize) {
        if self.literal_children.len() == 0 {
            return (self, 0);
        }
        (self, 0)
    }

    pub fn insert(&mut self, pattern: &CompiledPattern<'a, 'b>) -> Result<&'static str, &'static str> {
        if let Some(value) = pattern.first() {
            match value {
                &NodeType::Literal(literal) => {
                    return self.insert_literal(literal);
                },
                &NodeType::Parser(ref parser) => {
                    unimplemented!();
                }
            }
        } else {
            unimplemented!();
        }
        Err("err")
    }

    fn insert_literal(&mut self, literal: &'c str) -> Result<&'static str, &'static str> {
        let lit_node = LiteralNode::new(literal);
        if let Some(hit_pos) = self.literal_children.find_pos(&lit_node) {
            if let Some(prefix_len) = self.literal_children.get(hit_pos).unwrap().literal().has_common_prefix(literal) {
                let hit: LiteralNode<'a, 'b> = self.literal_children.remove(hit_pos);
                let common_prefix: &str = &literal[0..prefix_len];
                let left_branch: &str = &literal[prefix_len..];
                let right_branch: &str = &hit.literal()[prefix_len..];
                //let new_node = literal::split(hit, common_prefix, left_branch, right_branch);
            } else {
                unreachable!("There is a bug in the CommonPrefix implementation for str, or in LiteralNode's find() funciton")
            }
        } else {
            unimplemented!();
        }
        Err("err")
    }
}
