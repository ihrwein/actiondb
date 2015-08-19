use matcher::trie::node::{Node, TokenType};
use matcher::trie::{HasPattern, TrieOperations};
use matcher::result::MatchResult;
use matcher::Pattern;

#[derive(Debug, Clone)]
pub struct ParserTrie {
    root: Node,
}

macro_rules! insert_recurse {
    ($node:expr, $pattern:expr) => {
        {
            if !$pattern.has_more_tokens() {
                $node.set_pattern($pattern);
                $node
            } else {
                ParserTrie::insert_pattern($node, $pattern)
            }
        }
    }
}

impl ParserTrie {
    pub fn new() -> ParserTrie {
        ParserTrie{ root: Node::new() }
    }

    pub fn insert(&mut self, pattern: Pattern) -> &mut TrieOperations {
        if !pattern.has_more_tokens() {
            &mut self.root
        } else {
            ParserTrie::insert_pattern(&mut self.root, pattern)
        }
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
        self.root.parse(text)
    }

    fn insert_pattern<'a>(node: &'a mut TrieOperations, mut pattern: Pattern) -> &'a mut TrieOperations {
        match pattern.pop_first_token() {
            TokenType::Literal(literal) => {
                let mut node = node.insert_literal(&literal);
                insert_recurse!(node, pattern)
            },
            TokenType::Parser(parser) => {
                let mut node = node.insert_parser(parser);
                insert_recurse!(node, pattern)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use matcher::trie::node::{CompiledPattern, TokenType};
    use matcher::trie::ParserTrie;
    use parsers::{SetParser, IntParser, GreedyParser};
    use matcher::pattern::Pattern;

    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        let mut trie = ParserTrie::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(TokenType::Literal("app".to_string()));
        cp1.push(TokenType::Parser(Box::new(SetParser::from_str("test", "01234"))));
        cp1.push(TokenType::Literal("le".to_string()));

        {
            let mut pattern = Pattern::with_random_uuid();
            pattern.set_pattern(cp1);
            trie.insert(pattern);
            println!("{:?}", &trie);
        }
        {
            let mut pattern = Pattern::with_random_uuid();
            let mut cp2 = CompiledPattern::new();
            cp2.push(TokenType::Literal("appletree".to_string()));
            pattern.set_pattern(cp2);
            trie.insert(pattern);
        }
    }

    #[test]
    fn test_given_pattern_when_inserted_into_the_parser_tree_then_the_pattern_is_stored_in_the_leaf() {
        let mut trie = ParserTrie::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(TokenType::Literal("app".to_string()));
        cp1.push(TokenType::Parser(Box::new(SetParser::from_str("test", "01234"))));
        cp1.push(TokenType::Literal("le".to_string()));

        let mut pattern = Pattern::with_random_uuid();
        pattern.set_pattern(cp1);

        trie.insert(pattern);
        println!("{:?}", &trie);

        match trie.parse("app23le") {
            Some(res) => {
                println!("{:?}", res);
                assert_eq!(res.pairs(), &vec!(("test", "23")));
            },
            None => unreachable!()
        }
    }

    #[test]
    fn test_given_pattern_with_two_neighbouring_parser_when_the_pattern_is_inserted_into_the_trie_then_everything_is_ok() {
        let mut trie = ParserTrie::new();
        let mut cp1 = CompiledPattern::new();
        let mut expected = vec!(("test", "ccc"), ("test2", "12"), ("test3", "le"));
        expected.sort();
        cp1.push(TokenType::Literal("app".to_string()));
        cp1.push(TokenType::Parser(Box::new(SetParser::from_str("test", "abcd"))));
        cp1.push(TokenType::Parser(Box::new(IntParser::from_str("test2"))));
        cp1.push(TokenType::Parser(Box::new(GreedyParser::new("test3".to_string()))));

        let mut pattern = Pattern::with_random_uuid();
        pattern.set_pattern(cp1);

        trie.insert(pattern);
        println!("{:?}", &trie);

        match trie.parse("appccc12le") {
            Some(res) => {
                let mut got = res.pairs().clone();
                got.sort();
                println!("{:?}", res);
                assert_eq!(expected, got);
            },
            None => unreachable!()
        }
    }
}
