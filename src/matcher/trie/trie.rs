use matcher::trie::node::{Node, NodeType, LiteralNode};
use matcher::trie::node::CompiledPattern;

pub struct PatternTrie<'a> {
    root: Node<'a>,
}

impl <'a, 'b> PatternTrie<'a> {
    pub fn new() -> PatternTrie<'a> {
        PatternTrie{ root: Node::new() }
    }

    pub fn insert(&mut self, pattern: CompiledPattern<'a, 'b>) -> Result<&'static str, &'static str>{
        for i in pattern.into_iter() {
            match i {
                NodeType::Literal(literal) => {
                    let lnode = self.root.insert_literal(literal);
                },
                NodeType::Parser(parser) => {
                    unimplemented!();
                }
            }
        }
        Err("err")
    }
}

#[cfg(test)]
mod test {
    use matcher::trie::node::{CompiledPattern, NodeType, Node};
    use parsers::{Parser, SetParser};

    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        /*let mut root = Node::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(NodeType::Literal("app"));
        cp1.push(NodeType::Parser(Box::new(SetParser::new("01234"))));
        cp1.push(NodeType::Literal("le"));

        let mut cp2 = CompiledPattern::new();
        cp2.push(NodeType::Literal("applause"));

        root.insert(cp1);
        root.insert(cp2);*/
    }
}
