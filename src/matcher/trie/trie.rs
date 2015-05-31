use matcher::trie::node::{Node, NodeType, LiteralNode};
use matcher::trie::node::CompiledPattern;

pub struct PatternTrie<'a> {
    root: Node<'a>,
}

impl <'a, 'b> PatternTrie<'a> {
    pub fn new() -> PatternTrie<'a> {
        PatternTrie{ root: Node::new() }
    }

    pub fn insert(&mut self, pattern: CompiledPattern<'a, 'b>) -> Result<&'static str, &'static str> {
        self.root.insert(pattern)
    }
}
