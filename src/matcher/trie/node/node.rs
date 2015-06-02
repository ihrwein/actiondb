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
    Parser(Box<Parser<'a>>),
    Literal(&'b str)
}

#[derive(Debug)]
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

    fn lookup_literal_recurse(&mut self, literal: &str) -> Result<Option<(&mut Node<'a>, usize)>, Option<(&mut Node<'a>, usize)>> {
        println!("lookup_literal_recurse(): stepped in");
        let cmp_str = |x: &LiteralNode| {
            x.cmp_str(literal)
        };

        match self.literal_children.binary_search_by(&cmp_str) {
            Ok(pos) => {
                let elements_found = self.literal_children.get(pos).unwrap().literal().len();

                if !self.literal_children.get(pos).unwrap().is_leaf() {
                    if let Some(node) = self.literal_children.get_mut(pos).unwrap().node_mut() {
                        println!("lookup_literal(): going deeper");
                        node.lookup_literal(literal.ltrunc(elements_found))
                    } else {
                        unreachable!();
                    }
                } else {
                    println!("lookup_literal(): we found a prefix, but it's a leaf");
                    if self.literal_children.get(pos).unwrap().literal() == literal {
                        Ok(Some((self, 0)))
                    } else {
                        Err(Some((self, literal.len())))
                    }
                }
            },
            Err(pos) => {
                println!("lookup_literal(): there is no common prefix with this literal");
                Err(Some((self, literal.len())))
            }
        }
    }

    pub fn lookup_literal(&mut self, literal: &str) -> Result<Option<(&mut Node<'a>, usize)>, Option<(&mut Node<'a>, usize)>> {
        if !self.is_leaf() && !literal.is_empty() {
            println!("lookup_literal(): it's not a leaf nor is empty");
            self.lookup_literal_recurse(literal)
        } else if self.is_leaf() && literal.is_empty() {
            println!("lookup_literal(): we got it");
            Ok(Some((self, literal.len())))
        } else {
            println!("lookup_literal(): we can't go deeper");
            Err(Some((self, literal.len())))
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
        println!("inserting literal: '{}'", literal);
        let place_to_insert = self.lookup_literal(literal);

        match place_to_insert {
            Ok(option) => {
                match option {
                    Some(tuple) => {
                        println!("remaining len: {}", tuple.1);
                    },
                    None => {
                        println!("I should insert it here");
                    }
                }
            },
            Err(option) => {
                match option {
                    Some(tuple) => {
                        println!("remaining len: {}", tuple.1);
                        tuple.0.add_literal_node(LiteralNode::from_str(literal.ltrunc(literal.len() - tuple.1)));
                    },
                    None => {
                        //self.add_literal_node(LiteralNode::from_str(literal));
                        println!("doesn't have a parent");
                    }                }
            }
        }
        Err("asdas")
    }
}

#[test]
fn given_empty_trie_when_literals_are_inserted_then_they_can_be_looked_up() {
    let mut node = Node::new();

    node.insert_literal("alma");
    assert_eq!(node.lookup_literal("alma").is_ok(), true);
    assert_eq!(node.lookup_literal("alm").is_ok(), false);
    node.insert_literal("alma");
    assert_eq!(node.lookup_literal("alm").is_ok(), true);
}
