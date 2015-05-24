use std::collections::BTreeMap;
use std::borrow::ToOwned;
use std::iter::FromIterator;
use std::slice::Iter;
use std::vec::IntoIter as VecIntoIter;

use parsers::{Parser, SetParser, ParseResult};

struct Node<'a, 'b> {
    literal: String,
    child_parsers: Vec<Box<Parser<'a, 'b>>>,
    child_nodes: Vec<Box<Node<'a, 'b>>>
}

type MatchResult<'a> = Option<BTreeMap<&'a str, &'a str>>;

impl <'a, 'b> Node<'a, 'b> {
    pub fn new(literal: &str) -> Node<'a, 'b> {
        Node{ literal: literal.to_owned(),
              child_parsers: vec!(),
              child_nodes: vec!()}
    }

    pub fn new_root() -> Node<'a, 'b> {
        Node::new("")
    }

    pub fn add_child_parser(&mut self, parser: Box<Parser<'a, 'b>>) {
        self.child_parsers.push(parser);
    }

    pub fn add_child_node(&mut self, node: Box<Node<'a, 'b>>) {
        self.child_nodes.push(node);
    }

    pub fn parse(&mut self, value: &str) -> MatchResult {
        None
    }
}


enum NodeType<'a, 'b> {
    Parser(Box<Parser<'a, 'b>>),
    Literal(String)
}

type CompiledPattern<'a, 'b> = Vec<NodeType<'a, 'b>>;

#[test]
fn test_given_pattern_when_iterated_on_it_yields_expected_items() {
    let mut cp = CompiledPattern::new();
    let pn = Box::new(SetParser::new("0123456789"));

    cp.push(NodeType::Literal("alma".to_owned()));
    cp.push(NodeType::Parser(pn));
    cp.push(NodeType::Literal("fa".to_owned()));
    cp.push(NodeType::Parser(Box::new(SetParser::new("0123456789"))));

    for i in cp {
    }
}

#[test]
fn it_works() {
    let mut root = Node::new_root();
    let alma = Node::new("alma");
    let bela = Node::new("bela");
    let p = Box::new(SetParser::new("123a"));
    root.add_child_parser(p);
    root.add_child_node(Box::new(alma));
    root.add_child_node(Box::new(bela));
}
