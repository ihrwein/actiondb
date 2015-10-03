mod set;
mod base;
mod int;
mod length_checked;
mod greedy;

use std::fmt::Debug;
pub use self::set::SetParser;
pub use self::base::ParserBase;
pub use self::int::IntParser;
pub use self::length_checked::LengthCheckedParserBase;
pub use self::greedy::GreedyParser;

pub trait ObjectSafeHash {
    fn hash_os(&self) -> u64;
}

pub trait Parser: Debug + ObjectSafeHash {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<ParseResult<'a, 'b>>;
    fn name(&self) -> Option<&str>;
    fn set_name(&mut self, Option<String>);
    fn boxed_clone(&self) -> Box<Parser>;
}

pub trait HasOptionalParameter {
    fn set_optional_params<'a>(&mut self, params: Option<Vec<OptionalParameter<'a>>>) -> bool;
}

#[derive(Debug)]
pub enum OptionalParameter<'a> {
    Int(&'a str, usize),
}

#[derive(Debug)]
pub struct ParseResult<'a, 'b> {
    parser: &'a Parser,
    value: &'b str
}

impl<'a, 'b> ParseResult<'a, 'b> {
    pub fn new(parser: &'a Parser, value: &'b str) -> ParseResult<'a, 'b> {
        ParseResult {
            parser: parser,
            value: value
        }
    }

    pub fn parser(&self) -> &'a Parser {
        self.parser
    }

    pub fn value(&self) -> &'b str {
        self.value
    }
}

pub trait ParserFactory: {
    fn new_set<'a>(set: &str, name: Option<&str>, opt_params: Option<Vec<OptionalParameter<'a>>>) -> Box<Parser>;
    fn new_int<'a>(name: Option<&str>, opt_params: Option<Vec<OptionalParameter<'a>>>) -> Box<Parser>;
    fn new_greedy<'a>(name: Option<&str>, end_string: Option<&str>) -> Box<Parser>;
}
