use matcher::trie::node::{Node, TokenType};
use matcher::trie::node::{CompiledPattern};
use matcher::trie::{HasPattern, TrieOperations};
use matcher::result::MatchResult;
use matcher::Pattern;

#[derive(Debug, Clone)]
pub struct ParserTrie {
    root: Node,
}

macro_rules! insert_recurse {
    ($node:expr, $cp:expr, $pattern:expr) => {
        {
            if $cp.is_empty() {
                $node.set_pattern($pattern);
                $node
            } else {
                ParserTrie::insert_pattern($node, $cp, $pattern)
            }
        }
    }
}

impl ParserTrie {
    pub fn new() -> ParserTrie {
        ParserTrie{ root: Node::new() }
    }

    pub fn insert(&mut self, compiled_pattern: CompiledPattern, pattern: Pattern) -> &mut TrieOperations {
        if compiled_pattern.is_empty() {
            &mut self.root
        } else {
            ParserTrie::insert_pattern(&mut self.root, compiled_pattern, pattern)
        }
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
        self.root.parse(text)
    }

    fn insert_pattern<'a>(node: &'a mut TrieOperations, mut compiled_pattern: CompiledPattern, pattern: Pattern) -> &'a mut TrieOperations {
        let item = compiled_pattern.remove(0);
        match item {
            TokenType::Literal(literal) => {
                let mut node = node.insert_literal(&literal);
                insert_recurse!(node, compiled_pattern, pattern)
            },
            TokenType::Parser(parser) => {
                let mut node = node.insert_parser(parser);
                insert_recurse!(node, compiled_pattern, pattern)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use matcher::trie::node::{CompiledPattern, TokenType};
    use matcher::trie::ParserTrie;
    use parsers::{SetParser};
    use matcher::pattern::Pattern;
    use uuid::Uuid;

    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        let mut trie = ParserTrie::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(TokenType::Literal("app".to_string()));
        cp1.push(TokenType::Parser(Box::new(SetParser::from_str("test", "01234"))));
        cp1.push(TokenType::Literal("le".to_string()));

        trie.insert(cp1, Pattern::new(Uuid::new_v4()));
        println!("{:?}", &trie);

        let mut cp2 = CompiledPattern::new();
        cp2.push(TokenType::Literal("appletree".to_string()));
        trie.insert(cp2, Pattern::new(Uuid::new_v4()));
    }

    #[test]
    fn test_given_pattern_when_inserted_into_the_parser_tree_then_the_pattern_is_stored_in_the_leaf() {
        let mut trie = ParserTrie::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(TokenType::Literal("app".to_string()));
        cp1.push(TokenType::Parser(Box::new(SetParser::from_str("test", "01234"))));
        cp1.push(TokenType::Literal("le".to_string()));

        trie.insert(cp1, Pattern::new(Uuid::new_v4()));
        println!("{:?}", &trie);

        match trie.parse("app23le") {
            Some(res) => {
                println!("{:?}", res);
                assert_eq!(res.pairs(), &vec!(("test", "23")));
            },
            None => unreachable!()
        }
    }
}
