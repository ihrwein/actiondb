use std::collections::BTreeMap;
use std::borrow::ToOwned;
use std::iter::FromIterator;
use std::slice::Iter;
use std::vec::IntoIter as VecIntoIter;

use parsers::ParserNode;

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
