use std::collections::BTreeMap;
use std::cmp::Ordering;
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

    pub fn insert(&mut self, pattern: CompiledPattern<'a, 'b>) -> Result<&'static str, &'static str> {
        for i in pattern.into_iter() {
            match i {
                NodeType::Literal(literal) => {
                    return self.insert_literal(literal);
                },
                NodeType::Parser(parser) => {
                    unimplemented!();
                }
            }
        }
        Err("err")
    }

    fn insert_literal(&mut self, literal: &str) -> Result<&'static str, &'static str> {
        let cmp_str = |x: &LiteralNode| {
            x.cmp_str(literal)
        };

        if let Ok(hit_pos) = self.literal_children.binary_search_by(&cmp_str) {
            if let Some(common_prefix_len) = self.literal_children.get(hit_pos).unwrap().literal().has_common_prefix(&literal) {
                let hit: LiteralNode<'a, 'b> = self.literal_children.remove(hit_pos);
                let new_node = hit.split(common_prefix_len, literal);
                self.add_literal_node(new_node);
                return Ok("splitted");
            } else {
                unreachable!("There is a bug in the CommonPrefix implementation for str, or in LiteralNode's find() funciton")
            }
        } else {
            unimplemented!();
        }
        Err("err")
    }
}