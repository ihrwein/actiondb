pub mod trie;
mod errors;

pub use self::errors::BuildFromFileError;

use std::fs::File;
use std::io::{BufReader, BufRead};
use grammar::parser;
use grammar::parser::ParseError;
use self::trie::ParserTrie;

#[derive(Clone)]
pub struct Matcher {
    parser: ParserTrie
}

impl Matcher {
    pub fn from_file(pattern_file_path: &str) -> Result<Matcher, BuildFromFileError> {
        let file = try!(File::open(pattern_file_path));
        Matcher::build_matcher_from_file(&file)
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<Vec<(&'a str, &'b str)>> {
        self.parser.parse(text)
    }

    fn build_matcher_from_file(file: &File) -> Result<Matcher, BuildFromFileError> {
        let trie =  try!(Matcher::build_trie_from_file(&file));
        Ok(Matcher{ parser: trie })
    }

    fn build_trie_from_file(file: &File) -> Result<ParserTrie, parser::ParseError> {
        let mut trie = ParserTrie::new();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(l) = line {
                let compiled_pattern = try!(parser::pattern(&l));
                trie.insert(compiled_pattern);
            }
        }

        Ok(trie)
    }
}
