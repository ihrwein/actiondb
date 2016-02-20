mod set;
mod base;
mod int;
pub mod has_length_constraint;
mod greedy;

use std::fmt::Debug;
pub use self::set::SetParser;
pub use self::base::ParserBase;
pub use self::int::IntParser;
pub use self::has_length_constraint::HasLengthConstraint;
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

#[derive(Debug)]
pub enum OptionalParameter<'a> {
    Int(&'a str, usize),
}

#[derive(Debug)]
pub struct ParseResult<'a, 'b> {
    parser: &'a Parser,
    value: &'b str,
}

impl<'a, 'b> ParseResult<'a, 'b> {
    pub fn new(parser: &'a Parser, value: &'b str) -> ParseResult<'a, 'b> {
        ParseResult {
            parser: parser,
            value: value,
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
    fn new_set<'a>(set: &str,
                   name: Option<&str>,
                   opt_params: Option<Vec<OptionalParameter<'a>>>)
                   -> Box<Parser>;
    fn new_int(name: Option<&str>,
                   opt_params: Option<Vec<OptionalParameter>>)
                   -> Box<Parser>;
    fn new_greedy(name: Option<&str>, end_string: Option<&str>) -> Box<Parser>;
}
