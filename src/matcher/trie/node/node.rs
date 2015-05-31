use std::collections::BTreeMap;
use std::cmp::Ordering;
use parsers::{Parser, SetParser};
use utils::{SortedVec, CommonPrefix};
use matcher::trie::node::LiteralNode;
use matcher::trie::node::ParserNode;
use matcher::trie::node::literal;

pub type MatchResult<'a, 'b> = Option<BTreeMap<&'a str, &'b str>>;
pub type CompiledPattern<'a, 'b> = Vec<NodeType<'a, 'b>>;
type InsertResult<'a, 'b> = Result<&'a mut Node<'a>, &'static str>;

pub enum NodeType<'a, 'b> {
    Parser(Box<Parser<'a>>),
    Literal(&'b str)
}

pub struct Node<'a> {
    literal_children: SortedVec<LiteralNode<'a>>,
    parser_children: Vec<ParserNode<'a>>
}

impl <'a, 'b> Node<'a> {
    pub fn new() -> Node<'a> {
        Node{ literal_children: SortedVec::new(),
              parser_children: Vec::new() }
    }

    pub fn parse(&mut self, value: &'b str) -> MatchResult<'a, 'b> {
        None
    }

    pub fn add_literal_node(&mut self, lnode: LiteralNode<'a>) {
        self.literal_children.push(lnode);
    }

    fn find(&mut self, literal: &str) -> (&mut Node<'a>, usize) {
        if self.literal_children.len() == 0 {
            return (self, 0);
        }
        (self, 0)
    }

    pub fn insert(&mut self, pattern: CompiledPattern<'a, 'b>) -> Result<&'static str, &'static str>{
        for i in pattern.into_iter() {
            match i {
                NodeType::Literal(literal) => {
                    if let Ok(node) = self.insert_literal(literal) {
                    }
                },
                NodeType::Parser(parser) => {
                    unimplemented!();
                }
            }
        }
        Err("err")
    }

    fn insert_literal(&mut self, literal: &str) -> Result<Option<&mut Node<'a>>, &'static str> {
        let cmp_str = |x: &LiteralNode| {
            x.cmp_str(literal)
        };

        match self.literal_children.binary_search_by(&cmp_str) {
            Ok(hit_pos) => {
                if let Some(common_prefix_len) = self.literal_children.get(hit_pos).unwrap().literal().has_common_prefix(&literal) {
                    let hit: LiteralNode<'a> = self.literal_children.remove(hit_pos);
                    let new_node = hit.split(common_prefix_len, literal);
                    self.add_literal_node(new_node);
                    Ok(self.literal_children.get_mut(hit_pos).unwrap().node_mut())
                } else {
                    unreachable!("There is a bug in the CommonPrefix implementation for str, or in LiteralNode's find() funciton")
                }
            },
            Err(would_be_pos) => {
                let new_node = LiteralNode::from_str(literal);
                self.add_literal_node(new_node);
                Ok(self.literal_children.get_mut(would_be_pos).unwrap().node_mut())
            }
        }
    }
}
