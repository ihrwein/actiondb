use std::collections::BTreeMap;
use parsers::{Parser, SetParser};
use utils::{SortedVec, CommonPrefix};
use matcher::trie::node::LiteralNode;
use matcher::trie::node::ParserNode;
use matcher::trie::TrieOperations;

pub type MatchResult<'a, 'b> = Option<BTreeMap<&'a str, &'b str>>;
pub type CompiledPattern<'a> = Vec<NodeType<'a>>;

pub enum NodeType<'a> {
    Parser(Box<Parser>),
    Literal(&'a str)
}

#[derive(Debug)]
pub struct Node {
    literal_children: SortedVec<LiteralNode>,
    parser_children: Vec<ParserNode>
}

enum LiteralLookupResult<'a> {
    Found(usize),
    NotFound,
    GoDown(usize, &'a str)
}

impl Node {
    pub fn new() -> Node {
        Node{ literal_children: SortedVec::new(),
              parser_children: Vec::new() }
    }

    pub fn add_literal_node(&mut self, lnode: LiteralNode) {
        self.literal_children.push(lnode);
    }

    pub fn is_leaf(&self) -> bool {
        self.literal_children.is_empty() &&
            self.parser_children.is_empty()
    }


    // If a literal isn't found the last Node instance and the remaining length of the literal will be returned
    // if the literal is in the trie, we return the last Node instance and the index of the LiteralNode which contains the literal
    pub fn lookup_literal_mut(&mut self, literal: &str) -> Result<(&mut Node, usize), (&mut Node, usize)> {
        match self.search(literal) {
            LiteralLookupResult::Found(pos) => {
                Ok((self, pos))
            },
            LiteralLookupResult::NotFound => {
                Err((self, literal.len()))
            },
            LiteralLookupResult::GoDown(pos, truncated_literal) => {
                self.literal_children.get_mut(pos).unwrap().node_mut().unwrap().lookup_literal_mut(truncated_literal)
            },
        }
    }

    // It's the same as lookup_literal_mut() without the muts
    pub fn lookup_literal(&self, literal: &str) -> Result<(&Node, usize), (&Node, usize)> {
        match self.search(literal) {
            LiteralLookupResult::Found(pos) => {
                Ok((self, pos))
            },
            LiteralLookupResult::NotFound => {
                Err((self, literal.len()))
            },
            LiteralLookupResult::GoDown(pos, truncated_literal) => {
                self.literal_children.get(pos).unwrap().node().unwrap().lookup_literal(truncated_literal)
            },
        }
    }

    fn search<'a, 'b>(&'a self, literal: &'b str) -> LiteralLookupResult<'b> {
        println!("search(): stepped in");
        println!("search(): #children = {}", self.literal_children.len());
        println!("search(): #pchildren = {}", self.parser_children.len());
        let cmp_str = |probe: &LiteralNode| {
            probe.cmp_str(literal)
        };

        match self.literal_children.binary_search_by(&cmp_str) {
            Ok(pos) => {
                self.search_prefix_is_found(literal, pos)
            },
            Err(_) => {
                println!("search(): there is no common prefix with this literal");
                println!("search(): literal = {}", literal);
                println!("search(): #children = {}", self.literal_children.len());
                println!("search(): #pchildren = {}", self.parser_children.len());
                LiteralLookupResult::NotFound
            }
        }
    }

    fn search_prefix_is_found<'a, 'b>(&'a self, literal: &'b str, pos: usize) -> LiteralLookupResult<'b> {
        if !self.literal_children.get(pos).unwrap().is_leaf() {
            self.search_prefix_is_found_and_node_is_leaf(literal, pos)
        } else {
            self.search_prefix_is_found_and_node_is_not_leaf(literal, pos)
        }
    }

    fn search_prefix_is_found_and_node_is_not_leaf<'a, 'b>(&'a self, literal: &'b str, pos: usize) -> LiteralLookupResult<'b> {
        println!("search(): we found a prefix, but it's a leaf");
        if self.literal_children.get(pos).unwrap().literal() == literal {
            println!("search(): we got it");
            LiteralLookupResult::Found(pos)
        } else {
            println!("search(): we didn't get it");
            LiteralLookupResult::NotFound
        }
    }

    fn search_prefix_is_found_and_node_is_leaf<'a, 'b>(&'a self, literal: &'b str, pos: usize) -> LiteralLookupResult<'b> {
        let literal_node = self.literal_children.get(pos).unwrap();
        let common_prefix_len = literal_node.literal().common_prefix_len(literal);

        if common_prefix_len < literal_node.literal().len() {
            return LiteralLookupResult::NotFound;
        }

        if literal_node.has_value() && (literal.is_empty() || common_prefix_len == literal.len()) {
            println!("search(): we got it");
            return LiteralLookupResult::Found(pos);
        }

        if let Some(node) = literal_node.node() {
            println!("search(): literal len = {}", literal.len());
            println!("search(): common_prefix_len = {}", common_prefix_len);
            println!("search(): going deeper");
            return LiteralLookupResult::GoDown(pos, literal.ltrunc(common_prefix_len));
        } else {
            unreachable!();
        }
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<Vec<(&'a str, &'b str)>> {
        println!("parse(): text = {}", text);
        match self.lookup_literal(text) {
            Ok((node, pos)) => {
                Some(vec!())
            },
            Err((node, remaining_len)) => {
                let text = text.ltrunc(text.len() - remaining_len);
                println!("parse(): text = {}", text);
                println!("parse(): #parser_children = {}", node.parser_children.len());
                node.parse_with_parsers(text)
            }
        }
    }

    fn parse_with_parsers<'a, 'b>(&'a self, text: &'b str) -> Option<Vec<(&'a str, &'b str)>> {
        for i in self.parser_children.iter() {
            println!("parse(): testing parser");

            if let Some(vec) = i.parse(text) {
                return Some(vec);
            }
        }
        None
    }

    pub fn parse_then_push_kvpair<'a, 'b>(&'a self, text: &'b str, kvpair: (&'a str, &'b str)) -> Option<Vec<(&'a str, &'b str)>> {
        if let Some(mut vec) = self.parse(text) {
            vec.push(kvpair);
            Some(vec)
        } else {
            None
        }
    }

    fn lookup_parser(&mut self, parser: &Parser<>) -> Option<usize> {
        self.parser_children.iter().position(|ref x| x.parser().hash_os() == parser.hash_os())
    }

    fn insert_literal_tail(&mut self, tail: &str) -> &mut LiteralNode {
        println!("insert_literal_tail(): tail = {}", tail);
        let cmp_str = |probe: &LiteralNode| {
            probe.cmp_str(tail)
        };

        match self.literal_children.binary_search_by(&cmp_str) {
            Ok(pos) => {
                if let Some(common_prefix_len) = self.literal_children.get(pos).unwrap().literal().has_common_prefix(&tail) {
                    println!("insert_literal_tail(): common_prefix_len = {}", common_prefix_len);
                    let hit = self.literal_children.remove(pos);
                    println!("insert_literal_tail(): to_be_split = {}", hit.literal());
                    println!("insert_literal_tail(): tail = {}", tail);
                    let new_node = hit.split(common_prefix_len, tail);
                    self.add_literal_node(new_node);
                    self.literal_children.get_mut(pos).unwrap()
                } else {
                    unreachable!()
                }
            },
            Err(pos) => {
                println!("insert_literal_tail(): creating new literal node from tail = {}", tail);
                let mut new_node = LiteralNode::from_str(tail);
                new_node.set_has_value(true);
                self.add_literal_node(new_node);
                self.literal_children.get_mut(pos).unwrap()
            }
        }
    }
}

impl TrieOperations for Node {
    fn insert_literal(&mut self, literal: &str) -> &mut LiteralNode {
        println!("inserting literal: '{}'", literal);

        match self.lookup_literal_mut(literal) {
            Ok((node, index)) => {
                println!("insert_literal(): it was already inserted");
                node.literal_children.get_mut(index).unwrap()
            },
            Err((node, rem_len)) => {
                println!("INSERTING({}), remaining len: {}", literal, rem_len);
                let tail = literal.ltrunc(literal.len() - rem_len);
                node.insert_literal_tail(tail)
            }
        }
    }

    fn insert_parser(&mut self, parser: Box<Parser>) -> &mut ParserNode {
        if let Some(item) = self.lookup_parser(&*parser) {
            self.parser_children.get_mut(item).unwrap()
        } else {
            let pnode = ParserNode::new(parser);
            self.parser_children.push(pnode);
            self.parser_children.last_mut().unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use matcher::trie::{ParserTrie, TrieOperations};
    use parsers::{Parser, SetParser};
    use matcher::trie::node::{CompiledPattern, Node, NodeType};

    #[test]
    fn given_empty_trie_when_literals_are_inserted_then_they_can_be_looked_up() {
        let mut node = Node::new();

        let _ = node.insert_literal("alma");
        assert_eq!(node.lookup_literal("alma").is_ok(), true);
        assert_eq!(node.lookup_literal("alm").is_err(), true);
        let _ = node.insert_literal("alm");
        assert_eq!(node.lookup_literal("alm").is_ok(), true);
        assert_eq!(node.literal_children.len(), 1);
    }

    #[test]
    fn test_given_empty_trie_when_literals_are_inserted_the_child_counts_are_right() {
        let mut node = Node::new();

        let _ = node.insert_literal("alma");
        let _ = node.insert_literal("alm");
        assert_eq!(node.literal_children.len(), 1);
        assert_eq!(node.lookup_literal("alma").is_ok(), true);
        assert_eq!(node.lookup_literal("alm").ok().unwrap().0.literal_children.len(), 2);
    }

    #[test]
    fn test_given_empty_trie_when_literals_are_inserted_the_nodes_are_split_on_the_right_place() {
        let mut node = Node::new();

        let _ = node.insert_literal("alm");
        let _ = node.insert_literal("alma");
        let _ = node.insert_literal("ai");
        assert_eq!(node.literal_children.len(), 1);
        assert_eq!(node.lookup_literal("alma").is_ok(), true);
        assert_eq!(node.lookup_literal("alm").ok().unwrap().0.literal_children.len(), 2);
        assert_eq!(node.lookup_literal("ai").ok().unwrap().0.literal_children.len(), 2);
    }

    #[test]
    fn test_given_trie_when_literals_are_looked_up_then_the_edges_in_the_trie_are_not_counted_as_literals() {
        let mut node = Node::new();

        let _ = node.insert_literal("alm");
        let _ = node.insert_literal("ala");
        assert_eq!(node.lookup_literal("al").is_err(), true);
    }

    #[test]
    fn test_given_node_when_the_same_parsers_are_inserted_then_they_are_merged_into_one_parsernode() {
        let mut node = Node::new();

        let _ = node.insert_parser(Box::new(SetParser::from_str("test", "ab")));
        let _ = node.insert_parser(Box::new(SetParser::from_str("test", "ab")));

        assert_eq!(node.parser_children.len(), 1);
    }

    #[test]
    fn test_given_node_when_different_parsers_are_inserted_then_they_are_not_merged() {
        let mut node = Node::new();

        let _ = node.insert_parser(Box::new(SetParser::from_str("test", "ab")));
        let _ = node.insert_parser(Box::new(SetParser::from_str("test", "a")));

        assert_eq!(node.parser_children.len(), 2);
    }

    fn create_parser_trie() -> ParserTrie {
        let mut root = ParserTrie::new();
        let mut cp1 = CompiledPattern::new();
        let mut cp2 = CompiledPattern::new();
        let mut cp3 = CompiledPattern::new();
        cp1.push(NodeType::Literal("app"));
        cp1.push(NodeType::Parser(Box::new(SetParser::from_str("test", "01234"))));
        cp1.push(NodeType::Literal("le"));
        cp2.push(NodeType::Literal("appletree"));
        cp3.push(NodeType::Literal("apple"));

        root.insert(cp1);
        root.insert(cp2);
        root.insert(cp3);

        root
    }

    #[test]
    fn test_given_parser_trie_when_some_patterns_are_inserted_then_texts_can_be_parsed() {
        let root = create_parser_trie();

        println!("root: {:?}", &root);
        {
            let parsed_kwpairs = root.parse("bamboo");
            assert_eq!(parsed_kwpairs, None);
        }
        {
            let parsed_kwpairs = root.parse("app42le");
            assert_eq!(parsed_kwpairs.is_some(), true);
        }
    }

    #[test]
    fn test_given_parser_trie_when_some_patterns_are_inserted_then_fully_matching_literals_are_returned_as_empty_vectors() {
        let root = create_parser_trie();
        println!("root: {:?}", &root);
        {
            let parsed_kwpairs = root.parse("appletree");
            assert_eq!(parsed_kwpairs.unwrap().is_empty(), true);
        }
    }

    #[test]
    fn test_given_parser_trie_when_some_patterns_are_inserted_then_literal_matches_have_precedence_over_parser_matches() {
        let root = create_parser_trie();
        println!("root: {:?}", &root);
        {
            let parsed_kwpairs = root.parse("apple");
            assert_eq!(parsed_kwpairs.unwrap().is_empty(), true);
        }
    }

    fn create_complex_parser_trie() -> ParserTrie {
        let mut root = ParserTrie::new();
        let mut cp1 = CompiledPattern::new();
        let mut cp2 = CompiledPattern::new();
        let mut cp3 = CompiledPattern::new();
        let mut cp4 = CompiledPattern::new();
        cp1.push(NodeType::Literal("app"));
        cp1.push(NodeType::Parser(Box::new(SetParser::from_str("middle", "01234"))));
        cp1.push(NodeType::Literal("letree"));
        cp1.push(NodeType::Parser(Box::new(SetParser::from_str("end", "012"))));

        cp2.push(NodeType::Literal("app"));
        cp2.push(NodeType::Parser(Box::new(SetParser::from_str("middle", "01234"))));
        cp2.push(NodeType::Literal("letree"));
        cp2.push(NodeType::Parser(Box::new(SetParser::from_str("end", "0123"))));

        cp3.push(NodeType::Literal("bamboo"));

        cp4.push(NodeType::Literal("bamba"));

        root.insert(cp1);
        root.insert(cp2);
        root.insert(cp3);
        root.insert(cp4);

        root
    }

    #[test]
    fn test_given_parser_trie_when_a_parser_is_not_matched_then_the_parser_stack_is_unwind_so_an_untried_parser_is_tried() {
        let root = create_complex_parser_trie();
        println!("root: {:?}", &root);
        {
            let kvpairs = root.parse("app42letree123");
            assert_eq!(kvpairs.unwrap(), vec!(("end", "123"), ("middle", "42")));
        }
    }

    #[test]
    fn test_given_parser_trie_when_the_to_be_parsed_literal_is_not_matched_then_the_parse_result_is_none() {
        let root = create_complex_parser_trie();
        println!("root: {:?}", &root);
        {
            let kvpairs = root.parse("lorem ipsum");
            assert_eq!(kvpairs, None);
        }
    }

    #[test]
    fn test_given_parser_trie_when_the_to_be_parsed_literal_is_a_prefix_in_the_tree_then_the_parse_result_is_none() {
        let root = create_complex_parser_trie();
        println!("root: {:?}", &root);
        {
            let kvpairs = root.parse("bamb");
            assert_eq!(kvpairs, None);
        }
    }

    #[test]
    fn test_given_empty_parser_node_when_it_is_used_for_parsing_then_it_returns_none() {
        let root = Node::new();
        println!("root: {:?}", &root);
        {
            let kvpairs = root.parse("bamb");
            assert_eq!(kvpairs, None);
        }
        {
            let kvpairs = root.parse("");
            assert_eq!(kvpairs, None);
        }
    }
}
