use std::fs;
use std::io::Read;

use grammar::pattern_parser;
use matcher::trie::node::CompiledPattern;
use super::Error;

pub struct PlainPatternFile {
    patterns: Vec<CompiledPattern>
}

impl PlainPatternFile {
    pub fn open(path: &str) -> Result<PlainPatternFile, Error> {
        let mut file = try!(fs::File::open(path));
        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));
        let patterns = try!(pattern_parser::pattern_file(&buffer));
        Ok(PlainPatternFile{patterns: patterns})
    }

    pub fn patterns(self) -> Vec<CompiledPattern> {
        self.patterns
    }
}
