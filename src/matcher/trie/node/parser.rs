use matcher::trie::node::Node;
use parsers::Parser;

#[derive(Debug)]
pub struct ParserNode<'a> {
    parser: Box<Parser<'a>>,
    node: Option<Box<Node<'a>>>,
}

impl <'a> ParserNode<'a> {
    pub fn new(parser: Box<Parser<'a>>) -> ParserNode<'a> {
        ParserNode{ parser: parser,
                    node: None}
    }

    pub fn parser(&self) -> &Box<Parser<'a>> {
        &self.parser
    }
}
