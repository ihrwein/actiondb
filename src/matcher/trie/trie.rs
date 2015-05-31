use matcher::trie::node::{Node, NodeType, LiteralNode};
use matcher::trie::node::CompiledPattern;

pub struct PatternTrie<'a, 'b> {
    root: Node<'a, 'b>,
    patterns: Vec<CompiledPattern<'a, 'b>>
}

impl <'a, 'b> PatternTrie<'a, 'b> {
    pub fn new() -> PatternTrie<'a, 'b> {
        PatternTrie{ root: Node::new(),
                     patterns: vec!() }
    }

    pub fn insert(&mut self, pattern: CompiledPattern<'a, 'b>) -> Result<&'static str, &'static str> {
        self.patterns.push(pattern);
        self.root.insert(self.patterns.last().unwrap())
    }
}
