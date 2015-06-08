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
    pub fn lookup_literal(&mut self, literal: &str) -> Result<Option<(&mut Node, usize)>, Option<(&mut Node, usize)>> {
        println!("lookup_literal(): stepped in");
        println!("lookup_literal(): #children = {}", self.literal_children.len());
        let cmp_str = |probe: &LiteralNode| {
            probe.cmp_str(literal)
        };

        match self.literal_children.binary_search_by(&cmp_str) {
            Ok(pos) => {
                if !self.literal_children.get(pos).unwrap().is_leaf() {
                    let node_literal_len = self.literal_children.get(pos).unwrap().literal().len();
                    let common_prefix_len = self.literal_children.get(pos).unwrap().literal().common_prefix_len(literal);

                    if common_prefix_len < node_literal_len {
                        return Err(Some((self, literal.len())));
                    }

                    if literal.is_empty() && self.literal_children.get(pos).unwrap().has_value() {
                        println!("lookup_literal(): we got it, it's empty");
                        return Ok(Some((self, pos)));
                    }

                    if common_prefix_len == literal.len() &&
                        self.literal_children.get(pos).unwrap().has_value() {
                        println!("lookup_literal(): we got it, it ends here");
                        return Ok(Some((self, pos)));
                    }

                    if let Some(node) = self.literal_children.get_mut(pos).unwrap().node_mut() {
                        println!("lookup_literal(): literal len = {}", literal.len());
                        println!("lookup_literal(): common_prefix_len = {}", common_prefix_len);
                        println!("lookup_literal(): going deeper");
                        node.lookup_literal(literal.ltrunc(common_prefix_len))
                    } else {
                        unreachable!();
                    }
                } else {
                    println!("lookup_literal(): we found a prefix, but it's a leaf");
                    if self.literal_children.get(pos).unwrap().literal() == literal  && self.literal_children.get(pos).unwrap().has_value(){
                        println!("lookup_literal(): we got it");
                        Ok(Some((self, pos)))
                    } else {
                        println!("lookup_literal(): we didn't get it");
                        Err(Some((self, literal.len())))
                    }
                }
            },
            Err(_) => {
                println!("lookup_literal(): there is no common prefix with this literal");
                println!("lookup_literal(): literal = {}", literal);
                println!("lookup_literal(): {:?}", self);
                Err(Some((self, literal.len())))
            }
        }
    }

    pub fn parse<'a, 'b>(&'a mut self, text: &'b str) -> Option<Vec<(&'a str, &'b str)>> {
        match self.lookup_literal(text) {
            Ok(Some((node, len))) | Err(Some((node, len))) => {
                if len == 0 {
                    return Some(vec!());
                } else {
                    for i in node.parser_children.iter() {
                        let parsed_kwpair = i.parser().parse(text);

                        if parsed_kwpair.is_some() {
                            let vec = node.parse(text);
                        }

                    }
                }
            },
            _ => unreachable!()
        }
        None
    }

    fn parse_with_parsers<'a, 'b>(&'a mut self, text: &'b str) -> Option<(&'a str, &'b str)> {
        None
    }

    fn lookup_parser(&mut self, parser: &Parser<>) -> Option<usize> {
        self.parser_children.iter().position(|ref x| x.parser().hash_os() == parser.hash_os())
    }

    fn insert_literal_tail(&mut self, tail: &str) -> &mut LiteralNode {
        println!("insert_literal_tail(): tail = {}", tail);
        println!("{:?}", self);
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
                    println!("splitted");
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

        match self.lookup_literal(literal) {
            Ok(option) => {
                println!("insert_literal(): it was already inserted");
                let (node, index) = option.unwrap();
                node.literal_children.get_mut(index).unwrap()
            },
            Err(Some(tuple)) => {
                println!("INSERTING({}), remaining len: {}", literal, tuple.1);
                let tail = literal.ltrunc(literal.len() - tuple.1);
                tuple.0.insert_literal_tail(tail)
            },
            _ => {
                unreachable!();
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
    assert_eq!(node.lookup_literal("alm").ok().unwrap().unwrap().0.literal_children.len(), 2);
}

#[test]
#[no_mangle]
fn test_given_empty_trie_when_literals_are_inserted_the_nodes_are_split_on_the_right_place() {
    let mut node = Node::new();

    let _ = node.insert_literal("alm");
    let _ = node.insert_literal("alma");
    let _ = node.insert_literal("ai");
    assert_eq!(node.literal_children.len(), 1);
    assert_eq!(node.lookup_literal("alma").is_ok(), true);
    assert_eq!(node.lookup_literal("alm").ok().unwrap().unwrap().0.literal_children.len(), 2);
    assert_eq!(node.lookup_literal("ai").ok().unwrap().unwrap().0.literal_children.len(), 2);
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

    let _ = node.insert_parser(Box::new(SetParser::new("test", "ab")));
    let _ = node.insert_parser(Box::new(SetParser::new("test", "ab")));

    assert_eq!(node.parser_children.len(), 1);
}

#[test]
fn test_given_node_when_different_parsers_are_inserted_then_they_are_not_merged() {
    let mut node = Node::new();

    let _ = node.insert_parser(Box::new(SetParser::new("test", "ab")));
    let _ = node.insert_parser(Box::new(SetParser::new("test", "a")));

    assert_eq!(node.parser_children.len(), 2);
}


#[test]
fn test_given_parser_trie_when_some_patterns_are_inserted_then_texts_can_be_parsed() {
    let mut root = Node::new();
    let mut cp1 = CompiledPattern::new();
    let mut cp2 = CompiledPattern::new();
    cp1.push(NodeType::Literal("app"));
    cp1.push(NodeType::Parser(Box::new(SetParser::new("test", "01234"))));
    cp1.push(NodeType::Literal("le"));
    cp2.push(NodeType::Literal("appletree"));

    root.insert(cp1);
    root.insert(cp2);

    {
        let parsed_kwpairs = root.parse("bamboo");
        assert_eq!(parsed_kwpairs, None);
    }
    {
        let parsed_kwpairs = root.parse("app42le");
        assert_eq!(parsed_kwpairs.is_some(), true);
    }
}
