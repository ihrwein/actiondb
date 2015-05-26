mod literal;
mod parser;

use std::collections::BTreeMap;
use std::borrow::ToOwned;

use parsers::{Parser, SetParser};
use self::literal::LiteralNode;
use self::parser::ParserNode;

type MatchResult<'a, 'b> = Option<BTreeMap<&'a str, &'b str>>;
type CompiledPattern<'a, 'b> = Vec<NodeType<'a, 'b>>;

enum NodeType<'a, 'b> {
    Parser(Box<Parser<'a, 'b>>),
    Literal(String)
}

struct Node<'a, 'b> {
    literal_children: Vec<Box<LiteralNode<'a, 'b>>>,
    parser_children: Vec<Box<ParserNode<'a, 'b>>>
}

impl <'a, 'b, 'c> Node<'a, 'b> {
    pub fn new() -> Node<'a, 'b> {
        Node{ literal_children: vec!(),
              parser_children: vec!() }
    }

    pub fn parse(&'a mut self, value: &'b str) -> MatchResult<'c, 'b> {
        None
    }

    pub fn add_pattern(&'a mut self, pattern: &'a CompiledPattern<'a, 'b>) -> bool {
        false
    }
}
