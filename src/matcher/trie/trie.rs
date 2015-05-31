use matcher::trie::node::{Node, NodeType, LiteralNode};
use matcher::trie::node::CompiledPattern;

pub struct PatternTrie<'a, 'b> {
    root: Node<'a, 'b>,
}

impl <'a, 'b> PatternTrie<'a, 'b> {
    pub fn new() -> PatternTrie<'a, 'b> {
        PatternTrie{ root: Node::new() }
    }

    pub fn insert(&mut self, pattern: CompiledPattern<'a, 'b>) -> Result<&'static str, &'static str> {
        self.root.insert(pattern)
    }
}
