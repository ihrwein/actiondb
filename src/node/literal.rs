use node::Node;

pub struct LiteralNode <'a, 'b> {
    literal: String,
    node: Option<Box<Node<'a, 'b>>>,
}

impl <'a, 'b> LiteralNode<'a, 'b> {
    pub fn new(literal: &str) -> LiteralNode<'a, 'b> {
        LiteralNode{ literal: literal.to_owned(),
                     node: None}
    }
}
