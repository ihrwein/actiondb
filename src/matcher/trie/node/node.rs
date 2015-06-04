use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::cmp;
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
        println!("lookup_literal_recurse(): #children = {}", self.literal_children.len());
        let cmp_str = |probe: &LiteralNode| {
            probe.cmp_str(literal)
        };

        match self.literal_children.binary_search_by(&cmp_str) {
            Ok(pos) => {
                if !self.literal_children.get(pos).unwrap().is_leaf() {
                    let node_literal_len = self.literal_children.get(pos).unwrap().literal().len();
                    let common_prefix_len = self.literal_children.get(pos).unwrap().literal().common_prefix_len(literal);

                    if common_prefix_len < node_literal_len {
                        return Err(Some((self, literal.len())));
                    }

                    if let Some(node) = self.literal_children.get_mut(pos).unwrap().node_mut() {
                        println!("lookup_literal()_recurse: literal len = {}", literal.len());
                        println!("lookup_literal()_recurse: common_prefix_len = {}", common_prefix_len);
                        println!("lookup_literal()_recurse: going deeper");
                        node.lookup_literal(literal.ltrunc(common_prefix_len))
                    } else {
                        unreachable!();
                    }
                } else {
                    println!("lookup_literal_recurse(): we found a prefix, but it's a leaf");
                    if self.literal_children.get(pos).unwrap().literal() == literal {
                        println!("lookup_literal_recurse(): we got it");
                        Ok(Some((self, pos)))
                    } else {
                        println!("lookup_literal_recurse(): we didn't get it");
                        Err(Some((self, literal.len())))
                    }
                }
            },
            Err(pos) => {
                println!("lookup_literal_recurse(): there is no common prefix with this literal");
                println!("lookup_literal_recurse(): {:?}", self);
                Err(Some((self, literal.len())))
            }
        }
    }

    pub fn lookup_literal(&mut self, literal: &str) -> Result<Option<(&mut Node<'a>, usize)>, Option<(&mut Node<'a>, usize)>> {
        println!("lookup_literal(): looking up '{}'", literal);
        if !self.is_leaf() && !literal.is_empty() {
            println!("lookup_literal(): it's not a leaf nor is empty");
            self.lookup_literal_recurse(literal)
        } else if literal.is_empty() {
            println!("lookup_literal(): we got it");
            Ok(Some((self, 0)))
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

    fn insert_literal_tail(&mut self, tail: &str) {
        println!("insert_literal_tail(): tail = {}", tail);
        println!("{:?}", self);
        let cmp_str = |probe: &LiteralNode| {
            probe.cmp_str(tail)
        };

        match self.literal_children.binary_search_by(&cmp_str) {
            Ok(pos) => {
                if let Some(common_prefix_len) = self.literal_children.get(pos).unwrap().literal().has_common_prefix(&tail) {
                    println!("insert_literal_tail(): common_prefix_len = {}", common_prefix_len);
                    let hit = self.literal_children.remove(pos);
                    println!("insert_literal_tail(): to_be_split = {}", hit.literal());
                    println!("insert_literal_tail(): tail = {}", tail);
                    let new_node = hit.split(common_prefix_len, tail);
                    self.add_literal_node(new_node);
                    println!("splitted");
                } else {
                    unreachable!()
                }
            },
            Err(pos) => {
                println!("insert_literal_tail(): creating new literal node from tail = {}", tail);
                self.add_literal_node(LiteralNode::from_str(tail));
            }
        };

    }

    fn insert_literal(&mut self, literal: &str) -> Result<Option<&mut Node<'a>>, &'static str> {
        println!("inserting literal: '{}'", literal);

        match self.lookup_literal(literal) {
            Ok(option) => {
                println!("insert_literal(): it was already inserted");
                return Ok(Some(option.unwrap().0));
            },
            Err(Some(tuple)) => {
                println!("INSERTING({}), remaining len: {}", literal, tuple.1);
                let tail = literal.ltrunc(literal.len() - tuple.1);
                tuple.0.insert_literal_tail(tail);
            },
            _ => {
                unreachable!();
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
    assert_eq!(node.lookup_literal("alm").is_err(), true);
    node.insert_literal("alm");
    assert_eq!(node.lookup_literal("alm").is_ok(), true);
    assert_eq!(node.literal_children.len(), 1);
}

#[test]
fn test_given_empty_trie_when_literals_are_inserted_the_child_counts_are_right() {
    let mut node = Node::new();

    node.insert_literal("alma");
    node.insert_literal("alm");
    assert_eq!(node.literal_children.len(), 1);
    assert_eq!(node.lookup_literal("alma").is_ok(), true);
    assert_eq!(node.lookup_literal("alm").ok().unwrap().unwrap().0.literal_children.len(), 2);
}

#[test]
#[no_mangle]
fn test_given_empty_trie_when_literals_are_inserted_the_nodes_are_split_on_the_right_place() {
    let mut node = Node::new();

    node.insert_literal("alm");
    node.insert_literal("alma");
    node.insert_literal("ai");
    assert_eq!(node.literal_children.len(), 1);
    assert_eq!(node.lookup_literal("alma").is_ok(), true);
    assert_eq!(node.lookup_literal("alm").ok().unwrap().unwrap().0.literal_children.len(), 2);
    assert_eq!(node.lookup_literal("ai").ok().unwrap().unwrap().0.literal_children.len(), 2);
}
