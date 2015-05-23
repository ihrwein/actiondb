use std::collections::{BTreeSet,
                       BTreeMap};
use std::borrow::ToOwned;
use std::iter::FromIterator;

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

#[test]
fn it_works() {
    let mut root = Node::new_root();
    let p = Box::new(SetParserNode::new("123a"));
    root.add_child_parser(p);
}
