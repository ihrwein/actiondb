use std::collections::BTreeMap;
use std::borrow::ToOwned;

use parsers::{Parser, SetParser};

type MatchResult<'a> = Option<BTreeMap<&'a str, &'a str>>;
type CompiledPattern<'a, 'b> = Vec<NodeType<'a, 'b>>;

enum NodeType<'a, 'b> {
    Parser(Box<Parser<'a, 'b>>),
    Literal(String)
}

struct Node<'a, 'b> {
    literal_children: Vec<Box<LiteralNode<'a, 'b>>>,
    parser_children: Vec<Box<ParserNode<'a, 'b>>>
}

impl <'a, 'b> Node<'a, 'b> {
    pub fn new() -> Node<'a, 'b> {
        Node{ literal_children: vec!(),
              parser_children: vec!() }
    }
}

struct LiteralNode <'a, 'b> {
    literal: String,
    node: Option<Box<Node<'a, 'b>>>,
}

impl <'a, 'b> LiteralNode<'a, 'b> {
    pub fn new(literal: &str) -> LiteralNode<'a, 'b> {
        LiteralNode{ literal: literal.to_owned(),
                     node: None}
    }
}

struct ParserNode<'a, 'b> {
    parser: Box<Parser<'a, 'b>>,
    node: Option<Box<Node<'a, 'b>>>,
}

impl <'a, 'b> ParserNode<'a, 'b> {
    pub fn new(parser: Box<Parser<'a, 'b>>) -> ParserNode<'a, 'b> {
        ParserNode{ parser: parser,
                    node: None}
    }
}
