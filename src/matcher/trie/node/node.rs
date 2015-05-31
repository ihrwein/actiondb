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

    pub fn add_literal_node(&mut self, lnode: LiteralNode<'a>) {
        self.literal_children.push(lnode);
    }

    pub fn is_leaf(&self) -> bool {
        self.literal_children.is_empty() &&
            self.parser_children.is_empty()
    }

    fn lookup_literal_recurse(&mut self, literal: &str) -> Result<&mut Node<'a>, &mut Node<'a>> {
        let cmp_str = |x: &LiteralNode| {
            x.cmp_str(literal)
        };

        match self.literal_children.binary_search_by(&cmp_str) {
            Ok(pos) => {
                let found_len = self.literal_children.get(pos).unwrap().literal().len();

                if self.literal_children.get_mut(pos).unwrap().is_leaf() {
                    let mut literal_node = self.literal_children.get_mut(pos).unwrap();
                    literal_node.node_mut().unwrap().lookup_literal(literal.ltrunc(found_len))
                } else if self.literal_children.get_mut(pos).unwrap().literal() == literal {
                    Ok(self)
                } else {
                    Err(self)
                }
            },
            Err(pos) => {
                Err(self)
            }
        }
    }

    pub fn lookup_literal(&mut self, literal: &str) -> Result<&mut Node<'a>, &mut Node<'a>> {
        if !self.is_leaf() && !literal.is_empty() {
            self.lookup_literal_recurse(literal)
        } else if self.is_leaf() && literal.is_empty() {
            Ok(self)
        } else {
            Err(self)
        }
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
        let place_to_insert = self.lookup_literal(literal);

        Err("asd")
    }
}
