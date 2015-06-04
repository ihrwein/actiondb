mod set;

use std::fmt::Debug;
use std::hash::{Hash, Hasher};
pub use self::set::SetParser;

#[derive(PartialEq, Debug)]
pub enum ParseResult<'a> {
    Parsed(&'a str),
    NotParsed
}

pub trait Parser<'a>: Debug + Hash {
    fn parse(&self, value: &'a str) -> ParseResult<'a>;
}
