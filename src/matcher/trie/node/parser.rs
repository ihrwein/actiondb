use matcher::trie::node::Node;
use parsers::Parser;

pub struct ParserNode<'a, 'b> {
    parser: Box<Parser<'a, 'b>>,
    node: Option<Box<Node<'a, 'b>>>,
}

impl <'a, 'b> ParserNode<'a, 'b> {
    pub fn new(parser: Box<Parser<'a, 'b>>) -> ParserNode<'a, 'b> {
        ParserNode{ parser: parser,
                    node: None}
    }
}
