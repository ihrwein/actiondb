mod set;

use std::fmt::Debug;
pub use self::set::SetParser;

#[derive(PartialEq, Debug)]
pub enum ParseResult<'a> {
    Parsed(&'a str),
    NotParsed
}

pub trait ObjectSafeHash {
    fn hash_os(&self) -> u64;
}

pub trait Parser<'a>: Debug + ObjectSafeHash {
    fn parse(&self, value: &'a str) -> ParseResult<'a>;
}
