use std::collections::BTreeSet;
use std::borrow::ToOwned;

struct Node {
    literal: String,
    childParsers: Vec<Box<ParserNode>>,
    childNodes: Vec<Box<Node>>
}

impl Node {
    pub fn new(literal: &str) -> Node {
        Node{ literal: literal.to_owned(),
              childParsers: vec!(),
              childNodes: vec!()}
    }
}

trait ParserNode {

    fn parse(&self, value: &str) -> bool;

}

struct SetParserNode {
    characterSet: BTreeSet<u8>,
    minLength: Option<i32>,
    maxLength: Option<i32>
}

impl ParserNode for SetParserNode {

    fn parse(&self, value: &str) -> bool {
        true
    }
}

#[test]
fn it_works() {
    let mut root = Node::new("alma");
}
