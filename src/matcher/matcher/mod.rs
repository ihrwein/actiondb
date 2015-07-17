use uuid::Uuid;

use std::fs;
use std::io::{BufReader, BufRead};
use grammar::parser;
use grammar::parser::ParseError;
use super::trie::ParserTrie;
use super::result::MatchResult;
use super::errors::BuildFromFileError;
use super::pattern::Pattern;
use super::pattern::file;

mod builder;

#[derive(Clone, Debug)]
pub struct Matcher {
    parser: ParserTrie
}

impl Matcher {
    pub fn from_file(pattern_file_path: &str) -> Result<Matcher, BuildFromFileError> {
        let file = try!(fs::File::open(pattern_file_path));
        let trie = try!(Matcher::build_trie_from_file(&file));
        Ok(Matcher{ parser: trie })
    }

    pub fn from_json_file(pattern_file_path: &str) -> Result<Matcher, builder::BuildError> {
        let file = try!(file::SerializedPatternFile::open(pattern_file_path));
        let mut trie = ParserTrie::new();
        try!(builder::Builder::drain_into(&mut file.into_iter(), &mut trie));
        Ok(Matcher{ parser: trie })
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
        self.parser.parse(text)
    }

    fn build_trie_from_file(file: &fs::File) -> Result<ParserTrie, parser::ParseError> {
        let mut trie = ParserTrie::new();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(l) = line {
                let compiled_pattern = try!(parser::pattern(&l));
                let mut pattern = Pattern::with_uuid(Uuid::new_v4());
                pattern.set_pattern(compiled_pattern);
                trie.insert(pattern);
            }
        }

        Ok(trie)
    }
}
