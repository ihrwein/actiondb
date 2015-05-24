use std::collections::BTreeMap;
use std::borrow::ToOwned;
use std::iter::FromIterator;
use std::slice::Iter;
use std::vec::IntoIter as VecIntoIter;

mod parsers;
mod node;

use parsers::SetParserNode;

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
