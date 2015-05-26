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

    pub fn add_pattern(&'a mut self, pattern: CompiledPattern<'a, 'b>) {
    }
}

#[cfg(test)]
mod test {
    use node::{CompiledPattern, NodeType, Node};
    use parsers::{Parser, SetParser};

    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        let mut root = Node::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(NodeType::Literal("app".to_owned()));
        cp1.push(NodeType::Parser(Box::new(SetParser::new("01234"))));
        cp1.push(NodeType::Literal("le".to_owned()));

        let mut cp2 = CompiledPattern::new();
        cp2.push(NodeType::Literal("applause".to_owned()));

        root.add_pattern(cp1);
        root.add_pattern(cp2);
    }
}
