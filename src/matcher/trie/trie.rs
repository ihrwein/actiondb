use matcher::trie::node::{Node, NodeType};
use matcher::trie::node::CompiledPattern;

pub struct PatternTrie<'a, 'b> {
    root: Node<'a, 'b>
}

impl <'a, 'b> PatternTrie<'a, 'b> {
    pub fn new() -> PatternTrie<'a, 'b> {
        PatternTrie{ root: Node::new() }
    }

    pub fn insert(&mut self, pattern: CompiledPattern) -> Result<&str, &str> {
        for i in pattern.into_iter() {
            match i {
                NodeType::Literal(literal) => {
                    let pos = self.root.literal_children.find_pos(&literal);
                },
                NodeType::Parser(boxed_parser) => {

                }
            }
        }


        Err("err")
    }
}
