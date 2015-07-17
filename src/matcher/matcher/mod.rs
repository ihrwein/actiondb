use uuid::Uuid;

use std::fs;
use std::io::{BufReader, BufRead};
use grammar::parser;
use grammar::parser::ParseError;
use super::trie::ParserTrie;
use super::result::MatchResult;
use super::errors::{BuildFromFileError, FromJsonError};
use super::pattern::Pattern;
use super::pattern::file;
use super::pattern::testmessage::TestMessage;

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

    pub fn from_json_file(pattern_file_path: &str) -> Result<Matcher, FromJsonError> {
        let file = try!(file::SerializedPatternFile::open(pattern_file_path));
        let trie = try!(Matcher::build_trie_from_json_file(file));
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
                let mut pattern = Pattern::new(Uuid::new_v4());
                pattern.set_pattern(compiled_pattern);
                trie.insert(pattern);
            }
        }

        Ok(trie)
    }

    fn build_trie_from_json_file(file: file::SerializedPatternFile) -> Result<ParserTrie, FromJsonError> {
        let mut trie = ParserTrie::new();
        let file::SerializedPatternFile {patterns} = file;

        for pattern in patterns.into_iter() {
            try!(Matcher::insert_pattern_then_check_its_test_messages(&mut trie, pattern));
        }

        Ok(trie)
    }

    fn insert_pattern_then_check_its_test_messages(trie: &mut ParserTrie, mut pattern: Pattern) -> Result<(), FromJsonError> {
        let test_messages = Matcher::extract_test_messages_from_pattern(&mut pattern);
        trie.insert(pattern);
        Matcher::check_test_messages_on_trie(&trie, &test_messages)
    }

    fn extract_test_messages_from_pattern(pattern: &mut Pattern) -> Vec<TestMessage> {
        let mut messages = Vec::new();

        while let Some(test_message) = pattern.pop_test_message() {
            messages.push(test_message);
        }
        messages
    }

    fn check_test_messages_on_trie(trie: &ParserTrie, messages: &[TestMessage]) -> Result<(), FromJsonError> {
        for msg in messages {
            if let Some(result) = trie.parse(msg.message()) {
                try!(msg.test_pairs(result.pairs()));
            } else {
                return Err(FromJsonError::TestMessageDoesntMatch);
            }
        }
        Ok(())
    }
}
