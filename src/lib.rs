use std::collections::{BTreeSet,
                       BTreeMap};
use std::borrow::ToOwned;
use std::iter::FromIterator;
use std::slice::Iter;
use std::vec::IntoIter as VecIntoIter;

enum MatchResult {
    Partial{ length: i32 },
    Full{ length: i32 },
    None
}

struct Node {
    literal: String,
    childParsers: Vec<Box<ParserNode>>,
    childNodes: Vec<Box<Node>>
}

type ParseResult<'a> = Option<BTreeMap<&'a str, &'a str>>;

impl Node {
    pub fn new(literal: &str) -> Node {
        Node{ literal: literal.to_owned(),
              childParsers: vec!(),
              childNodes: vec!()}
    }

    pub fn new_root() -> Node {
        Node::new("")
    }

    pub fn add_child_parser(&mut self, parser: Box<ParserNode>) {
        self.childParsers.push(parser);
    }

    pub fn add_child_node(&mut self, node: Box<Node>) {
        self.childNodes.push(node);
    }

    pub fn parse(&mut self, value: &str) -> ParseResult {
        None
    }
}

trait ParserNode {

    fn parse(&self, value: &str) -> MatchResult;

}

struct SetParserNode {
    characterSet: BTreeSet<u8>,
    minLength: Option<i32>,
    maxLength: Option<i32>
}

impl SetParserNode {
    fn create_set_from_str(set: &str) -> BTreeSet<u8> {
        let vset: Vec<u8> = set.bytes().collect();
        BTreeSet::from_iter(vset)
    }

    pub fn new(set: &str) -> SetParserNode {
        SetParserNode{ characterSet: SetParserNode::create_set_from_str(set),
                        minLength: None,
                        maxLength: None}
    }
}

impl ParserNode for SetParserNode {
    fn parse(&self, value: &str) -> MatchResult {
        MatchResult::None
    }
}

enum NodeType {
    Parser(Box<ParserNode>),
    Literal(String)
}

struct CompiledPattern {
    pattern: Vec<NodeType>
}

impl CompiledPattern {
    pub fn new() -> CompiledPattern {
        CompiledPattern{ pattern: vec!() }
    }

    pub fn push_parser(&mut self, parser: Box<ParserNode>) {
        self.pattern.push(NodeType::Parser(parser));
    }

    pub fn push_literal(&mut self, literal: String) {
        self.pattern.push(NodeType::Literal(literal));
    }
}

impl IntoIterator for CompiledPattern {
    type Item = NodeType;
    type IntoIter = VecIntoIter<NodeType>;

    fn into_iter(self) -> Self::IntoIter {
        self.pattern.into_iter()
    }
}

#[test]
fn test_given_pattern_when_iterated_on_it_yields_expected_items() {
    let mut cp = CompiledPattern::new();
    let pn = Box::new(SetParserNode::new("0123456789"));

    cp.push_literal("alma".to_owned());
    cp.push_parser(pn);
    cp.push_literal("fa".to_owned());

    for i in cp.pattern.into_iter() {
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
