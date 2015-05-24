use std::collections::BTreeMap;
use std::borrow::ToOwned;
use std::iter::FromIterator;
use std::slice::Iter;
use std::vec::IntoIter as VecIntoIter;

use parsers::{ParserNode, SetParserNode, MatchResult};

struct Node {
    literal: String,
    child_parsers: Vec<Box<ParserNode>>,
    child_nodes: Vec<Box<Node>>
}

type ParseResult<'a> = Option<BTreeMap<&'a str, &'a str>>;

impl Node {
    pub fn new(literal: &str) -> Node {
        Node{ literal: literal.to_owned(),
              child_parsers: vec!(),
              child_nodes: vec!()}
    }

    pub fn new_root() -> Node {
        Node::new("")
    }

    pub fn add_child_parser(&mut self, parser: Box<ParserNode>) {
        self.child_parsers.push(parser);
    }

    pub fn add_child_node(&mut self, node: Box<Node>) {
        self.child_nodes.push(node);
    }

    pub fn parse(&mut self, value: &str) -> ParseResult {
        None
    }
}


enum NodeType {
    Parser(Box<ParserNode>),
    Literal(String)
}

type CompiledPattern = Vec<NodeType>;

#[test]
fn test_given_pattern_when_iterated_on_it_yields_expected_items() {
    let mut cp = CompiledPattern::new();
    let pn = Box::new(SetParserNode::new("0123456789"));

    cp.push(NodeType::Literal("alma".to_owned()));
    cp.push(NodeType::Parser(pn));
    cp.push(NodeType::Literal("fa".to_owned()));
    cp.push(NodeType::Parser(Box::new(SetParserNode::new("0123456789"))));

    for i in cp {
    }
}

#[test]
fn it_works() {
    let mut root = Node::new_root();
    let alma = Node::new("alma");
    let bela = Node::new("bela");
    let p = Box::new(SetParserNode::new("123a"));
    root.add_child_parser(p);
    root.add_child_node(Box::new(alma));
    root.add_child_node(Box::new(bela));
}
