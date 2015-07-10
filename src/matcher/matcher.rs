use uuid::Uuid;
use yaml_rust::yaml;


use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use grammar::parser;
use grammar::parser::ParseError;
use super::trie::ParserTrie;
use super::result::MatchResult;
use super::errors::BuildFromFileError;
use super::pattern::Pattern;

#[derive(Clone)]
pub struct Matcher {
    parser: ParserTrie
}

impl Matcher {
    pub fn from_file(pattern_file_path: &str) -> Result<Matcher, BuildFromFileError> {
        let file = try!(File::open(pattern_file_path));
        let trie = try!(Matcher::build_trie_from_file(&file));
        Ok(Matcher{ parser: trie })
    }

    pub fn from_yaml_file(pattern_file_path: &str) -> Result<Matcher, BuildFromFileError> {
        let mut file = try!(File::open(pattern_file_path));
        let trie = try!(Matcher::build_trie_from_yaml_file(&mut file));
        Ok(Matcher{ parser: trie })
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
        self.parser.parse(text)
    }

    fn build_trie_from_file(file: &File) -> Result<ParserTrie, parser::ParseError> {
        let mut trie = ParserTrie::new();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(l) = line {
                let compiled_pattern = try!(parser::pattern(&l));
                trie.insert(compiled_pattern, Pattern::new(Uuid::new_v4()));
            }
        }

        Ok(trie)
    }

    fn build_trie_from_yaml_file(file: &mut File) -> Result<ParserTrie, parser::ParseError> {
        let mut buffer = String::new();
        let mut trie = ParserTrie::new();

        file.read_to_string(&mut buffer).unwrap();
        let docs = yaml::YamlLoader::load_from_str(&buffer).unwrap();

        for doc in &docs {
            println!("{:?}", doc);
            //dump_node(doc, 0);
        }
        Ok(trie)
    }
}
